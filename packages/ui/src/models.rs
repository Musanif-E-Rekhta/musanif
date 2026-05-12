use serde::{Deserialize, Serialize};

// ── Identity ──────────────────────────────────────────────────────────────────

/// Mirrors `UserResponseGql` on the backend — the user record returned by
/// `login`, `register`, and the `user` field of `AuthPayload`. Profile data
/// (first_name, bio, avatar, …) lives on a separate `Profile` row keyed by
/// `user_id`; query `me { profile { … } }` to get it.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(default)]
    pub is_active: bool,
    #[serde(default)]
    pub is_verified: bool,
    #[serde(default)]
    pub plan_tier: Option<String>,
}

/// Mirrors `ProfileResponseGql` — the editable per-user profile.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub tier: String,
    pub name: String,
    pub features: Vec<String>,
}

/// Composite response for the `me` query — bundles the User row with the
/// optional Profile and the user's plan.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserMe {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub profile: Option<Profile>,
    pub plan: Plan,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserStats {
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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReadingSessionResponse {
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

/// Returned by `login` and `register` mutations.
///
/// When `requires_2fa` is `true`, `token` is a short-lived (5 min) 2FA
/// challenge token (purpose `2fa_challenge`), not an access token; the
/// client must follow up with `login_2fa_complete(token, code)` to
/// exchange it for a session.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    pub token: String,
    /// Long-lived opaque token used by `refresh_token` to mint a new
    /// access JWT. `None` while the caller still owes a 2FA challenge.
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// async-graphql renames `requires_2fa` to `requires2Fa` (digit-boundary
    /// rule), so we explicitly map back here.
    #[serde(default, rename = "requires2Fa")]
    pub requires_2fa: bool,
    /// Short-lived 2FA challenge token; pass it back to
    /// `login_2fa_complete`. `None` outside the 2FA flow.
    #[serde(default)]
    pub challenge: Option<String>,
    pub user: User,
}

/// Current 2FA enrolment state for the signed-in user. `last_used_at`
/// is an ISO-8601 string when present; rendering decides format.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TwoFactorStatus {
    pub enabled: bool,
    pub last_used_at: Option<String>,
}

/// Returned by `setup_2fa` — the only time the plaintext secret and
/// recovery codes ever appear on the wire. The client renders the
/// `otpauth_url` as a QR code; the user confirms enrolment by calling
/// `verify_2fa(code, plaintext_recovery_codes)`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Setup2faPayload {
    pub secret: String,
    pub otpauth_url: String,
    pub qr_svg: String,
    pub recovery_codes: Vec<String>,
}

// ── Content ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    #[serde(default)]
    pub followers: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookAuthor {
    pub author: Author,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Book {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub isbn: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub page_count: Option<i32>,
    pub language: String,
    pub published_at: Option<String>,
    pub avg_rating: Option<f64>,
    #[serde(default)]
    pub review_count: i32,
    #[serde(default)]
    pub chapter_count: i32,
    pub is_published: bool,
    pub authors: Option<Vec<BookAuthor>>,
    pub categories: Option<Vec<Category>>,
    pub tags: Option<Vec<Tag>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedBook {
    pub book: Book,
    pub featured_until: Option<String>,
    pub eyebrow: Option<String>,
    pub headline: Option<String>,
    pub blurb: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookRef {
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChapterNav {
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
}

/// Minimal chapter response returned by the by-number redirect endpoint.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChapterSlugRef {
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub id: String,
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
    pub content: String,
    pub content_format: String,
    pub summary: Option<String>,
    pub meta_description: Option<String>,
    pub word_count: Option<i32>,
    pub reading_time_mins: Option<i32>,
    pub avg_rating: Option<f64>,
    #[serde(default)]
    pub review_count: i32,
    pub is_published: bool,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    pub prev_chapter: Option<ChapterNav>,
    pub next_chapter: Option<ChapterNav>,
    pub book: Option<BookRef>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChapterSummary {
    pub id: String,
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
    pub reading_time_mins: Option<i32>,
    pub avg_rating: Option<f64>,
    #[serde(default)]
    pub is_published: bool,
}

// ── Reading ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    pub id: String,
    pub book_id: String,
    pub status: String,
    pub progress: Option<i32>,
    pub notes: Option<String>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub updated_at: String,
    pub book: Option<Book>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReadingGoal {
    pub id: String,
    pub year: i32,
    pub target: i32,
    pub completed: i32,
    pub progress_pct: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub is_public: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CollectionBook {
    pub book_id: String,
    pub position: Option<i32>,
    pub note: Option<String>,
    pub added_at: String,
    pub book: Option<Book>,
}

// ── Annotations ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Highlight {
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
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub user_id: String,
    pub book_id: String,
    pub chapter_id: String,
    pub highlight_id: Option<String>,
    pub parent_id: Option<String>,
    pub body: String,
    pub is_spoiler: bool,
    pub offset_start: Option<i32>,
    pub offset_end: Option<i32>,
    pub text_snapshot: Option<String>,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

// ── Reviews ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BookReview {
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
    pub created_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ChapterReview {
    pub id: String,
    pub user_id: String,
    pub chapter_id: String,
    pub rating: i32,
    pub body: Option<String>,
    pub contains_spoiler: bool,
    pub helpful_count: i32,
    pub status: String,
    pub created_at: String,
}

// ── Translations ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WordTranslation {
    pub id: String,
    pub word: String,
    pub translation: String,
    pub source_lang: String,
    pub target_lang: String,
    pub submitted_by: String,
    pub scope: String,
    pub book_id: Option<String>,
    pub chapter_id: Option<String>,
    pub context_note: Option<String>,
    pub upvotes: i32,
    pub downvotes: i32,
    pub score: i32,
    pub created_at: String,
}

// ── Input types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertBookmarkInput {
    pub status: String,
    pub progress: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookReviewInput {
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: bool,
    pub reading_status: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookReviewInput {
    pub rating: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: Option<bool>,
    pub reading_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateChapterReviewInput {
    pub rating: i32,
    pub body: Option<String>,
    pub contains_spoiler: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagReviewInput {
    pub reason: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHighlightInput {
    pub offset_start: i32,
    pub offset_end: i32,
    pub paragraph: i32,
    pub text_snapshot: String,
    pub color: Option<String>,
    pub note: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHighlightInput {
    pub color: Option<String>,
    pub note: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentInput {
    pub body: String,
    pub is_spoiler: Option<bool>,
    pub highlight_id: Option<String>,
    pub parent_id: Option<String>,
    pub offset_start: Option<i32>,
    pub offset_end: Option<i32>,
    pub text_snapshot: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTranslationInput {
    pub word: String,
    pub translation: String,
    pub source_lang: String,
    pub target_lang: String,
    pub scope: String,
    pub book_slug: Option<String>,
    pub chapter_slug: Option<String>,
    pub context_note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionInput {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddBookToCollectionInput {
    pub book_slug: String,
    pub position: Option<i32>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertReadingGoalInput {
    pub year: i32,
    pub target: i32,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileInput {
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordInput {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgotPasswordInput {
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordInput {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordReadingSessionInput {
    pub book_slug: String,
    pub chapter_slug: Option<String>,
    pub started_at: String,
    pub ended_at: Option<String>,
    pub duration_mins: Option<i32>,
    pub page_start: Option<i32>,
    pub page_end: Option<i32>,
    pub device: Option<String>,
}
