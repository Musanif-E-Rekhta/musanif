//! Highlight create + list operations for the reader.
//!
//! The schema's `HighlightGql` shape is already defined in [`super::me`]
//! (used by `myHighlights` in the profile shelf), so this module reuses
//! it via re-export rather than declaring the type twice. Only the
//! input and mutation/query fragments are new here.

use cynic::{MutationBuilder, QueryBuilder};

use super::client::run;
use super::me::HighlightGql;
use super::schema::schema;
use crate::models;

// ── Inputs ────────────────────────────────────────────────────────────────────

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "CreateHighlightInput")]
pub struct CreateHighlightInputCynic {
    pub book_slug: String,
    pub chapter_slug: String,
    pub offset_start: i32,
    pub offset_end: i32,
    pub paragraph: i32,
    pub text_snapshot: String,
    pub color: Option<String>,
    pub note: Option<String>,
    pub is_public: Option<bool>,
}

// ── Variables ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateHighlightVars {
    pub input: CreateHighlightInputCynic,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ChapterHighlightsVars {
    pub book_slug: String,
    pub chapter_slug: String,
}

// ── Mutation / query fragments ────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "CreateHighlightVars")]
pub struct CreateHighlightMutation {
    #[arguments(input: $input)]
    pub create_highlight: HighlightGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "ChapterHighlightsVars")]
pub struct ChapterHighlightsQuery {
    #[arguments(bookSlug: $book_slug, chapterSlug: $chapter_slug)]
    pub chapter_highlights: Vec<HighlightGql>,
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Creates a highlight on the current chapter. Returns the persisted
/// record so the caller can render it without a refetch.
pub async fn create_highlight(
    book_slug: String,
    chapter_slug: String,
    paragraph: i32,
    offset_start: i32,
    offset_end: i32,
    text_snapshot: String,
    color: Option<String>,
    note: Option<String>,
    is_public: Option<bool>,
) -> Option<models::Highlight> {
    let op = CreateHighlightMutation::build(CreateHighlightVars {
        input: CreateHighlightInputCynic {
            book_slug,
            chapter_slug,
            offset_start,
            offset_end,
            paragraph,
            text_snapshot,
            color,
            note,
            is_public,
        },
    });
    run(op).await.map(|d| d.create_highlight.into())
}

/// Lists all of the signed-in user's own highlights for one chapter.
/// The schema also accepts a `public` flag for browsing other readers'
/// highlights, but the reader currently shows only the user's own.
pub async fn fetch_chapter_highlights(
    book_slug: String,
    chapter_slug: String,
) -> Option<Vec<models::Highlight>> {
    let op = ChapterHighlightsQuery::build(ChapterHighlightsVars {
        book_slug,
        chapter_slug,
    });
    run(op)
        .await
        .map(|d| d.chapter_highlights.into_iter().map(Into::into).collect())
}
