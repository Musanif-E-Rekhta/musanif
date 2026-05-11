//! Authenticated-user queries and mutations: `me`, stats, sessions,
//! following, highlights, bookmarks, reading goal, plus profile + password
//! mutations. Wraps cynic-typed operations and projects them into the
//! existing `models::*` shapes that views consume.

use chrono::{DateTime, Utc};
use cynic::{MutationBuilder, QueryBuilder};

use super::client::run;
use super::schema::schema;
use crate::models;

// ── Cynic types ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct PlanGql {
    pub tier: String,
    pub name: String,
    pub features: Vec<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ProfileResponseGql {
    pub id: String,
    pub user_id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub language: String,
    pub country: String,
    pub timezone: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct MeResponseGql {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub profile: Option<ProfileResponseGql>,
    pub plan: PlanGql,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct UserStatsGql {
    pub books_reading: i32,
    pub books_completed: i32,
    pub books_read_later: i32,
    pub books_dropped: i32,
    pub highlights_count: i32,
    pub reviews_count: i32,
    pub reading_sessions_count: i32,
    pub hours_read: f64,
    pub day_streak: i32,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ReadingSessionGql {
    pub id: String,
    pub book_id: String,
    pub chapter_id: Option<String>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub duration_mins: Option<i32>,
    pub page_start: i32,
    pub page_end: Option<i32>,
    pub device: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct AuthorGql {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct HighlightGql {
    pub id: String,
    pub user_id: String,
    pub book_id: String,
    pub chapter_id: String,
    pub offset_start: i32,
    pub offset_end: i32,
    pub paragraph: i32,
    pub text_snapshot: String,
    pub color: String,
    pub note: Option<String>,
    pub is_public: bool,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct BookmarkGql {
    pub id: String,
    pub book_id: String,
    pub status: String,
    pub progress: Option<i32>,
    pub notes: Option<String>,
    pub last_chapter_id: Option<String>,
    pub last_offset: Option<i32>,
    pub last_read_at: Option<String>,
    pub progress_pct: Option<f64>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ReadingGoalGql {
    pub id: String,
    pub year: i32,
    pub target: i32,
    pub completed: i32,
    pub progress_pct: f64,
    pub on_track: bool,
    pub pace_hint: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct TwoFactorStatusGql {
    pub enabled: bool,
    pub last_used_at: Option<DateTime<Utc>>,
}

// ── Variables ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct PaginationVars {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct OrderedPaginationVars {
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct BookmarksVars {
    pub status: Option<String>,
    pub order: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct YearVars {
    pub year: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpdateProfileVars {
    pub input: UpdateProfileInputCynic,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ChangePasswordVars {
    pub old_password: String,
    pub new_password: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct UpsertReadingGoalVars {
    pub year: i32,
    pub target: i32,
}

// ── Inputs ────────────────────────────────────────────────────────────────────

/// Mirrors the schema's `UpdateProfileInput`; cynic auto-camelCases the
/// fields, so snake_case Rust keeps the public API readable.
#[derive(cynic::InputObject, Debug, Default)]
#[cynic(graphql_type = "UpdateProfileInput")]
pub struct UpdateProfileInputCynic {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub timezone: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
}

impl From<models::UpdateProfileInput> for UpdateProfileInputCynic {
    fn from(i: models::UpdateProfileInput) -> Self {
        UpdateProfileInputCynic {
            first_name: i.first_name,
            last_name: i.last_name,
            display_name: i.display_name,
            avatar_url: i.avatar_url,
            bio: i.bio,
            language: i.language,
            country: i.country,
            timezone: i.timezone,
            phone: i.phone,
            website: i.website,
        }
    }
}

// ── Queries ───────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
pub struct MeQuery {
    pub me: MeResponseGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
pub struct MyStatsQuery {
    pub my_stats: UserStatsGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "PaginationVars")]
pub struct MyReadingSessionsQuery {
    #[arguments(limit: $limit, offset: $offset)]
    pub my_reading_sessions: Vec<ReadingSessionGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "PaginationVars")]
pub struct MyFollowingQuery {
    #[arguments(limit: $limit, offset: $offset)]
    pub my_following: Vec<AuthorGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "OrderedPaginationVars")]
pub struct MyHighlightsQuery {
    #[arguments(order: $order, limit: $limit, offset: $offset)]
    pub my_highlights: Vec<HighlightGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "BookmarksVars")]
pub struct MyBookmarksQuery {
    #[arguments(status: $status, order: $order, limit: $limit, offset: $offset)]
    pub my_bookmarks: Vec<BookmarkGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "YearVars")]
pub struct MyReadingGoalQuery {
    #[arguments(year: $year)]
    pub my_reading_goal: Option<ReadingGoalGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
pub struct My2FaStatusQuery {
    #[cynic(rename = "my2FaStatus")]
    pub my_2fa_status: TwoFactorStatusGql,
}

// ── Mutations ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "UpdateProfileVars")]
pub struct UpdateProfileMutation {
    #[arguments(input: $input)]
    pub update_profile: ProfileResponseGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "ChangePasswordVars")]
pub struct ChangePasswordMutation {
    #[arguments(oldPassword: $old_password, newPassword: $new_password)]
    pub change_password: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot")]
pub struct DeleteMeMutation {
    pub delete_me: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "UpsertReadingGoalVars")]
pub struct UpsertReadingGoalMutation {
    #[arguments(year: $year, target: $target)]
    pub upsert_reading_goal: ReadingGoalGql,
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<ProfileResponseGql> for models::Profile {
    fn from(p: ProfileResponseGql) -> Self {
        models::Profile {
            id: p.id,
            user_id: p.user_id,
            first_name: p.first_name,
            last_name: p.last_name,
            display_name: p.display_name,
            avatar_url: p.avatar_url,
            bio: p.bio,
            language: p.language,
            country: p.country,
            timezone: p.timezone,
            phone: p.phone,
            website: p.website,
        }
    }
}

impl From<PlanGql> for models::Plan {
    fn from(p: PlanGql) -> Self {
        models::Plan {
            tier: p.tier,
            name: p.name,
            features: p.features,
        }
    }
}

impl From<MeResponseGql> for models::UserMe {
    fn from(m: MeResponseGql) -> Self {
        models::UserMe {
            id: m.id,
            username: m.username,
            email: m.email,
            is_active: m.is_active,
            is_verified: m.is_verified,
            profile: m.profile.map(Into::into),
            plan: m.plan.into(),
        }
    }
}

/// Most views still consume the slimmer `User` type even when the full
/// `Me` payload is fetched. This conversion drops the profile/plan
/// extras for that legacy callsite.
impl From<MeResponseGql> for models::User {
    fn from(m: MeResponseGql) -> Self {
        models::User {
            id: m.id,
            username: m.username,
            email: m.email,
            is_active: m.is_active,
            is_verified: m.is_verified,
            plan_tier: Some(m.plan.tier),
        }
    }
}

impl From<UserStatsGql> for models::UserStats {
    fn from(s: UserStatsGql) -> Self {
        models::UserStats {
            books_reading: s.books_reading,
            books_completed: s.books_completed,
            books_read_later: s.books_read_later,
            books_dropped: s.books_dropped,
            highlights_count: s.highlights_count,
            reviews_count: s.reviews_count,
            reading_sessions_count: s.reading_sessions_count,
            hours_read: s.hours_read,
            day_streak: s.day_streak,
        }
    }
}

impl From<ReadingSessionGql> for models::ReadingSessionResponse {
    fn from(r: ReadingSessionGql) -> Self {
        models::ReadingSessionResponse {
            id: r.id,
            book_id: r.book_id,
            chapter_id: r.chapter_id,
            started_at: r.started_at,
            ended_at: r.ended_at,
            duration_mins: r.duration_mins,
            page_start: r.page_start,
            page_end: r.page_end,
            device: r.device,
        }
    }
}

impl From<AuthorGql> for models::Author {
    fn from(a: AuthorGql) -> Self {
        models::Author {
            id: a.id,
            name: a.name,
            slug: a.slug,
            bio: a.bio,
            avatar_url: a.avatar_url,
            website: a.website,
            followers: 0,
        }
    }
}

impl From<HighlightGql> for models::Highlight {
    fn from(h: HighlightGql) -> Self {
        models::Highlight {
            id: h.id,
            user_id: h.user_id,
            book_id: h.book_id,
            chapter_id: h.chapter_id,
            offset_start: h.offset_start,
            offset_end: h.offset_end,
            paragraph: h.paragraph,
            text_snapshot: h.text_snapshot,
            color: h.color,
            note: h.note,
            is_public: h.is_public,
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

impl From<BookmarkGql> for models::Bookmark {
    fn from(b: BookmarkGql) -> Self {
        models::Bookmark {
            id: b.id,
            book_id: b.book_id,
            status: b.status,
            progress: b.progress,
            notes: b.notes,
            started_at: None,
            completed_at: None,
            updated_at: b.last_read_at.unwrap_or_default(),
            book: None,
        }
    }
}

impl From<ReadingGoalGql> for models::ReadingGoal {
    fn from(g: ReadingGoalGql) -> Self {
        models::ReadingGoal {
            id: g.id,
            year: g.year,
            target: g.target,
            completed: g.completed,
            progress_pct: g.progress_pct,
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn fetch_me() -> Option<models::UserMe> {
    let op = MeQuery::build(());
    run(op).await.map(|d| d.me.into())
}

pub async fn fetch_my_stats() -> Option<models::UserStats> {
    let op = MyStatsQuery::build(());
    run(op).await.map(|d| d.my_stats.into())
}

pub async fn fetch_my_reading_sessions() -> Option<Vec<models::ReadingSessionResponse>> {
    let op = MyReadingSessionsQuery::build(PaginationVars {
        limit: None,
        offset: None,
    });
    run(op)
        .await
        .map(|d| d.my_reading_sessions.into_iter().map(Into::into).collect())
}

pub async fn fetch_my_following() -> Option<Vec<models::Author>> {
    let op = MyFollowingQuery::build(PaginationVars {
        limit: None,
        offset: None,
    });
    run(op)
        .await
        .map(|d| d.my_following.into_iter().map(Into::into).collect())
}

pub async fn fetch_my_highlights() -> Option<Vec<models::Highlight>> {
    let op = MyHighlightsQuery::build(OrderedPaginationVars {
        order: None,
        limit: None,
        offset: None,
    });
    run(op)
        .await
        .map(|d| d.my_highlights.into_iter().map(Into::into).collect())
}

pub async fn fetch_my_bookmarks(status: Option<String>) -> Option<Vec<models::Bookmark>> {
    let op = MyBookmarksQuery::build(BookmarksVars {
        status,
        order: None,
        limit: None,
        offset: None,
    });
    run(op)
        .await
        .map(|d| d.my_bookmarks.into_iter().map(Into::into).collect())
}

pub async fn fetch_my_reading_goal() -> Option<models::ReadingGoal> {
    let op = MyReadingGoalQuery::build(YearVars { year: None });
    run(op).await.and_then(|d| d.my_reading_goal.map(Into::into))
}

pub async fn update_profile(
    input: models::UpdateProfileInput,
) -> Option<models::Profile> {
    let op = UpdateProfileMutation::build(UpdateProfileVars {
        input: input.into(),
    });
    run(op).await.map(|d| d.update_profile.into())
}

pub async fn change_password(old_password: String, new_password: String) -> bool {
    let op = ChangePasswordMutation::build(ChangePasswordVars {
        old_password,
        new_password,
    });
    run(op)
        .await
        .map(|d| d.change_password)
        .unwrap_or(false)
}

pub async fn delete_me() -> bool {
    let op = DeleteMeMutation::build(());
    run(op).await.map(|d| d.delete_me).unwrap_or(false)
}

pub async fn upsert_reading_goal(year: i32, target: i32) -> Option<models::ReadingGoal> {
    let op = UpsertReadingGoalMutation::build(UpsertReadingGoalVars { year, target });
    run(op).await.map(|d| d.upsert_reading_goal.into())
}
