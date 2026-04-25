use serde::{Deserialize, Serialize};

// ── Identity ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct UserStats {
    pub bookmark_counts: std::collections::HashMap<String, i32>,
    pub highlight_count: i32,
    pub review_count: i32,
    pub session_count: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ReadingSessionResponse {
    pub id: String,
    pub book_id: String,
    pub book_title: String,
    pub chapter_id: String,
    pub chapter_number: i32,
    pub duration_mins: i32,
    pub occurred_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

// ── Content ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Publisher {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
pub struct BookAuthor {
    pub author: Author,
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
pub struct BookRef {
    pub title: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChapterNav {
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
}

/// Minimal chapter response returned by the by-number redirect endpoint.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChapterSlugRef {
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
pub struct ReadingGoal {
    pub id: String,
    pub year: i32,
    pub target: i32,
    pub completed: i32,
    pub progress_pct: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
pub struct CollectionBook {
    pub book_id: String,
    pub position: Option<i32>,
    pub note: Option<String>,
    pub added_at: String,
    pub book: Option<Book>,
}

// ── Annotations ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpsertBookmarkInput {
    pub status: String,
    pub progress: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateBookReviewInput {
    pub rating: i32,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: bool,
    pub reading_status: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateBookReviewInput {
    pub rating: Option<i32>,
    pub title: Option<String>,
    pub body: Option<String>,
    pub contains_spoiler: Option<bool>,
    pub reading_status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateChapterReviewInput {
    pub rating: i32,
    pub body: Option<String>,
    pub contains_spoiler: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FlagReviewInput {
    pub reason: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
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
pub struct UpdateHighlightInput {
    pub color: Option<String>,
    pub note: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
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
pub struct CreateCollectionInput {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateCollectionInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddBookToCollectionInput {
    pub book_slug: String,
    pub position: Option<i32>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpsertReadingGoalInput {
    pub year: i32,
    pub target: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateProfileInput {
    pub username: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChangePasswordInput {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForgotPasswordInput {
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResetPasswordInput {
    pub token: String,
    pub new_password: String,
}
