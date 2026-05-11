//! Mutations that take an `input: Foo` shape — bookmarks, reviews,
//! reading sessions, reading goals.

use chrono::{DateTime, Utc};
use cynic::MutationBuilder;

use super::client::run;
use super::me::BookmarkGql;
use super::schema::schema;
use crate::models;

// ── Cynic types (re-exporting nested fragments where they're shared) ─────────

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct BookReviewGql {
    pub id: String,
    pub user_id: String,
    pub book_id: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: bool,
    pub reading_status: String,
    pub verified_reader: bool,
    pub helpful_count: i32,
    pub status: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ReadingSessionGql {
    pub id: String,
}

// ── Inputs ────────────────────────────────────────────────────────────────────

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "UpsertBookmarkInput")]
pub struct UpsertBookmarkInputCynic {
    pub status: String,
    pub progress: Option<i32>,
    pub notes: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "CreateBookReviewInput")]
pub struct CreateBookReviewInputCynic {
    pub book_slug: String,
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: Option<bool>,
    pub reading_status: String,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "RecordReadingSessionInput")]
pub struct RecordReadingSessionInputCynic {
    pub book_slug: String,
    pub chapter_slug: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_mins: Option<i32>,
    pub page_start: Option<i32>,
    pub page_end: Option<i32>,
    pub device: Option<String>,
}

// ── Variables ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct UpsertBookmarkVars {
    pub book_slug: String,
    pub input: UpsertBookmarkInputCynic,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateBookReviewVars {
    pub input: CreateBookReviewInputCynic,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct RecordReadingSessionVars {
    pub input: RecordReadingSessionInputCynic,
}

// ── Mutations ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "UpsertBookmarkVars")]
pub struct UpsertBookmarkMutation {
    #[arguments(bookSlug: $book_slug, input: $input)]
    pub upsert_bookmark: BookmarkGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "CreateBookReviewVars")]
pub struct CreateBookReviewMutation {
    #[arguments(input: $input)]
    pub create_book_review: BookReviewGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "RecordReadingSessionVars")]
pub struct RecordReadingSessionMutation {
    #[arguments(input: $input)]
    pub record_reading_session: ReadingSessionGql,
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<BookReviewGql> for models::BookReview {
    fn from(r: BookReviewGql) -> Self {
        models::BookReview {
            id: r.id,
            user_id: r.user_id,
            book_id: r.book_id,
            rating: r.rating,
            title: r.title,
            body: r.body,
            contains_spoiler: r.contains_spoiler,
            reading_status: r.reading_status,
            verified_reader: r.verified_reader,
            helpful_count: r.helpful_count,
            status: r.status,
            created_at: String::new(),
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn upsert_bookmark(
    book_slug: String,
    status: String,
    progress: Option<i32>,
    notes: Option<String>,
) -> Option<models::Bookmark> {
    let op = UpsertBookmarkMutation::build(UpsertBookmarkVars {
        book_slug,
        input: UpsertBookmarkInputCynic {
            status,
            progress,
            notes,
        },
    });
    run(op).await.map(|d| d.upsert_bookmark.into())
}

pub async fn submit_book_review(
    book_slug: String,
    rating: i32,
    title: Option<String>,
    body: Option<String>,
    contains_spoiler: bool,
    reading_status: String,
) -> Option<models::BookReview> {
    let op = CreateBookReviewMutation::build(CreateBookReviewVars {
        input: CreateBookReviewInputCynic {
            book_slug,
            rating,
            title,
            body,
            contains_spoiler: Some(contains_spoiler),
            reading_status,
        },
    });
    run(op).await.map(|d| d.create_book_review.into())
}

pub async fn record_reading_session(input: models::RecordReadingSessionInput) -> bool {
    let started_at = match DateTime::parse_from_rfc3339(&input.started_at) {
        Ok(dt) => dt.with_timezone(&Utc),
        Err(_) => return false,
    };
    let ended_at = input
        .ended_at
        .as_deref()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&Utc));

    let op = RecordReadingSessionMutation::build(RecordReadingSessionVars {
        input: RecordReadingSessionInputCynic {
            book_slug: input.book_slug,
            chapter_slug: input.chapter_slug,
            started_at,
            ended_at,
            duration_mins: input.duration_mins,
            page_start: input.page_start,
            page_end: input.page_end,
            device: input.device,
        },
    });
    run(op).await.is_some()
}

