/// REST API client.
///
/// Build selection:
///   BUILD_PROFILE=local  (default) → reads .env.local
///   BUILD_PROFILE=prod             → reads .env.prod
///
/// Previously implemented (7):
///   fetch_books, fetch_book, fetch_chapters, fetch_chapter,
///   fetch_author, fetch_author_books, fetch_book_reviews, fetch_my_bookmarks
///
/// Added in this revision (49):
///   Auth:        register, login, logout, forgot_password, reset_password
///   Me:          fetch_me, update_profile, change_password, delete_me,
///                fetch_my_stats, fetch_my_reading_sessions, fetch_my_following
///   Books:       fetch_book_authors
///   Authors:     fetch_authors, follow_author, unfollow_author
///   Taxonomy:    fetch_categories, fetch_books_by_category, fetch_books_by_tag
///   Chapters:    fetch_chapter_by_number
///   Reviews:     submit_book_review, update_book_review, vote_book_review,
///                fetch_chapter_reviews, submit_chapter_review,
///                vote_chapter_review, flag_review
///   Highlights:  fetch_chapter_highlights, create_highlight,
///                fetch_my_highlights, update_highlight, delete_highlight
///   Comments:    fetch_chapter_comments, create_comment,
///                fetch_highlight_comments, update_comment,
///                delete_comment, vote_comment
///   Translations:fetch_word_translations, submit_translation, vote_translation
///   Collections: fetch_my_collections, create_collection, fetch_collection,
///                update_collection, delete_collection,
///                add_book_to_collection, remove_book_from_collection
///   Bookmarks:   upsert_bookmark, delete_bookmark
///   Goals:       fetch_my_reading_goal, upsert_reading_goal
use std::sync::{Mutex, OnceLock};

use reqwest::{Client, Method};
use serde::Serialize;

use crate::{config, models::*};

// ── Auth token store ──────────────────────────────────────────────────────────

static TOKEN: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn token_store() -> &'static Mutex<Option<String>> {
    TOKEN.get_or_init(|| Mutex::new(None))
}

/// Store (or clear) the bearer token used for authenticated requests.
pub fn set_auth_token(token: Option<String>) {
    if let Ok(mut g) = token_store().lock() {
        *g = token;
    }
}

/// Read the current bearer token.
pub fn get_auth_token() -> Option<String> {
    token_store().lock().ok()?.clone()
}

// ── Request helpers ───────────────────────────────────────────────────────────

fn url(path: &str) -> String {
    format!("{}{}", config::api_base_url(), path)
}

fn req(method: Method, path: &str) -> reqwest::RequestBuilder {
    let builder = Client::new().request(method, url(path));
    match get_auth_token() {
        Some(token) => builder.bearer_auth(token),
        None => builder,
    }
}

async fn get_json<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Option<T> {
    req(Method::GET, path).send().await.ok()?.json().await.ok()
}

async fn post_json<B: Serialize, T: for<'de> serde::Deserialize<'de>>(
    path: &str,
    body: &B,
) -> Option<T> {
    req(Method::POST, path)
        .json(body)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

async fn put_json<B: Serialize, T: for<'de> serde::Deserialize<'de>>(
    path: &str,
    body: &B,
) -> Option<T> {
    req(Method::PUT, path)
        .json(body)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

async fn delete_ok(path: &str) -> bool {
    req(Method::DELETE, path)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── Auth ──────────────────────────────────────────────────────────────────────

pub async fn register(input: RegisterInput) -> Option<AuthPayload> {
    post_json("/auth/register", &input).await
}

pub async fn login(input: LoginInput) -> Option<AuthPayload> {
    post_json("/auth/login", &input).await
}

pub async fn logout() -> bool {
    req(Method::POST, "/auth/logout")
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

pub async fn forgot_password(input: ForgotPasswordInput) -> bool {
    post_json::<_, User>("/api/v1/auth/forgot-password", &input)
        .await
        .is_some()
}

pub async fn reset_password(input: ResetPasswordInput) -> bool {
    post_json::<_, User>("/api/v1/auth/reset-password", &input)
        .await
        .is_some()
}

// ── Me ────────────────────────────────────────────────────────────────────────

pub async fn fetch_me() -> Option<User> {
    get_json("/api/v1/me").await
}

pub async fn update_profile(input: UpdateProfileInput) -> Option<User> {
    put_json("/api/v1/me/profile", &input).await
}

pub async fn change_password(input: ChangePasswordInput) -> bool {
    put_json::<_, User>("/api/v1/me/password", &input)
        .await
        .is_some()
}

pub async fn delete_me() -> bool {
    delete_ok("/api/v1/me").await
}

pub async fn fetch_my_stats() -> Option<UserStats> {
    get_json("/api/v1/me/stats").await
}

pub async fn fetch_my_reading_sessions() -> Option<Vec<ReadingSessionResponse>> {
    get_json("/api/v1/me/reading-sessions").await
}

pub async fn fetch_my_following() -> Option<Vec<Author>> {
    get_json("/api/v1/me/following").await
}

// ── Books ─────────────────────────────────────────────────────────────────────

pub async fn fetch_books(q: Option<String>, lang: Option<String>) -> Option<Vec<Book>> {
    let mut r = Client::new().get(url("/books"));
    if let Some(q) = q {
        r = r.query(&[("q", q)]);
    }
    if let Some(lang) = lang {
        r = r.query(&[("lang", lang)]);
    }
    r.send().await.ok()?.json().await.ok()
}

pub async fn fetch_book(slug: String) -> Option<Book> {
    get_json(&format!("/books/{slug}")).await
}

pub async fn fetch_book_authors(slug: String) -> Option<Vec<BookAuthor>> {
    get_json(&format!("/books/{slug}/authors")).await
}

// ── Authors ───────────────────────────────────────────────────────────────────

pub async fn fetch_authors(q: Option<String>) -> Option<Vec<Author>> {
    let mut r = Client::new().get(url("/authors"));
    if let Some(q) = q {
        r = r.query(&[("q", q)]);
    }
    r.send().await.ok()?.json().await.ok()
}

pub async fn fetch_author(slug: String) -> Option<Author> {
    get_json(&format!("/authors/{slug}")).await
}

pub async fn fetch_author_books(slug: String) -> Option<Vec<Book>> {
    get_json(&format!("/authors/{slug}/books")).await
}

pub async fn follow_author(slug: String) -> bool {
    req(Method::POST, &format!("/authors/{slug}/follow"))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

pub async fn unfollow_author(slug: String) -> bool {
    req(Method::DELETE, &format!("/authors/{slug}/follow"))
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── Categories & Tags ─────────────────────────────────────────────────────────

pub async fn fetch_categories() -> Option<Vec<Category>> {
    get_json("/categories").await
}

pub async fn fetch_books_by_category(slug: String) -> Option<Vec<Book>> {
    get_json(&format!("/categories/{slug}/books")).await
}

pub async fn fetch_books_by_tag(slug: String) -> Option<Vec<Book>> {
    get_json(&format!("/tags/{slug}/books")).await
}

// ── Chapters ──────────────────────────────────────────────────────────────────

pub async fn fetch_chapters(book_slug: String) -> Option<Vec<ChapterSummary>> {
    get_json(&format!("/books/{book_slug}/chapters")).await
}

pub async fn fetch_chapter(book_slug: String, chapter_slug: String) -> Option<Chapter> {
    get_json(&format!("/books/{book_slug}/chapters/{chapter_slug}")).await
}

/// Resolve a chapter number to its slug without a full chapter fetch.
pub async fn fetch_chapter_by_number(
    book_slug: String,
    number: i32,
) -> Option<ChapterSlugRef> {
    get_json(&format!("/books/{book_slug}/chapters/by-number/{number}")).await
}

// ── Reviews ───────────────────────────────────────────────────────────────────

pub async fn fetch_book_reviews(
    book_slug: String,
    spoilers: bool,
    limit: Option<i32>,
) -> Option<Vec<BookReview>> {
    Client::new()
        .get(url(&format!("/books/{book_slug}/reviews")))
        .query(&[
            ("spoilers", spoilers.to_string()),
            ("limit", limit.unwrap_or(10).to_string()),
        ])
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

pub async fn submit_book_review(
    book_slug: String,
    input: CreateBookReviewInput,
) -> Option<BookReview> {
    post_json(&format!("/books/{book_slug}/reviews"), &input).await
}

pub async fn update_book_review(
    book_slug: String,
    review_id: String,
    input: UpdateBookReviewInput,
) -> Option<BookReview> {
    put_json(&format!("/books/{book_slug}/reviews/{review_id}"), &input).await
}

pub async fn vote_book_review(book_slug: String, review_id: String, value: i32) -> bool {
    #[derive(Serialize)]
    struct Body {
        value: i32,
    }
    req(
        Method::POST,
        &format!("/books/{book_slug}/reviews/{review_id}/vote"),
    )
    .json(&Body { value })
    .send()
    .await
    .map(|r| r.status().is_success())
    .unwrap_or(false)
}

pub async fn fetch_chapter_reviews(
    book_slug: String,
    chapter_slug: String,
) -> Option<Vec<ChapterReview>> {
    get_json(&format!("/books/{book_slug}/chapters/{chapter_slug}/reviews")).await
}

pub async fn submit_chapter_review(
    book_slug: String,
    chapter_slug: String,
    input: CreateChapterReviewInput,
) -> Option<ChapterReview> {
    post_json(
        &format!("/books/{book_slug}/chapters/{chapter_slug}/reviews"),
        &input,
    )
    .await
}

pub async fn vote_chapter_review(
    book_slug: String,
    chapter_slug: String,
    review_id: String,
    value: i32,
) -> bool {
    #[derive(Serialize)]
    struct Body {
        value: i32,
    }
    req(
        Method::POST,
        &format!("/books/{book_slug}/chapters/{chapter_slug}/reviews/{review_id}/vote"),
    )
    .json(&Body { value })
    .send()
    .await
    .map(|r| r.status().is_success())
    .unwrap_or(false)
}

pub async fn flag_review(review_id: String, input: FlagReviewInput) -> bool {
    req(Method::POST, &format!("/reviews/{review_id}/flag"))
        .json(&input)
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── Highlights ────────────────────────────────────────────────────────────────

pub async fn fetch_chapter_highlights(
    book_slug: String,
    chapter_slug: String,
    public_only: bool,
) -> Option<Vec<Highlight>> {
    Client::new()
        .get(url(&format!(
            "/books/{book_slug}/chapters/{chapter_slug}/highlights"
        )))
        .query(&[("public", public_only.to_string())])
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

pub async fn create_highlight(
    book_slug: String,
    chapter_slug: String,
    input: CreateHighlightInput,
) -> Option<Highlight> {
    post_json(
        &format!("/books/{book_slug}/chapters/{chapter_slug}/highlights"),
        &input,
    )
    .await
}

pub async fn fetch_my_highlights() -> Option<Vec<Highlight>> {
    get_json("/me/highlights").await
}

pub async fn update_highlight(highlight_id: String, input: UpdateHighlightInput) -> Option<Highlight> {
    put_json(&format!("/highlights/{highlight_id}"), &input).await
}

pub async fn delete_highlight(highlight_id: String) -> bool {
    delete_ok(&format!("/highlights/{highlight_id}")).await
}

// ── Comments ──────────────────────────────────────────────────────────────────

pub async fn fetch_chapter_comments(
    book_slug: String,
    chapter_slug: String,
) -> Option<Vec<Comment>> {
    get_json(&format!("/books/{book_slug}/chapters/{chapter_slug}/comments")).await
}

pub async fn create_comment(
    book_slug: String,
    chapter_slug: String,
    input: CreateCommentInput,
) -> Option<Comment> {
    post_json(
        &format!("/books/{book_slug}/chapters/{chapter_slug}/comments"),
        &input,
    )
    .await
}

pub async fn fetch_highlight_comments(highlight_id: String) -> Option<Vec<Comment>> {
    get_json(&format!("/highlights/{highlight_id}/comments")).await
}

pub async fn update_comment(comment_id: String, body: String) -> Option<Comment> {
    #[derive(Serialize)]
    struct Body {
        body: String,
    }
    put_json(&format!("/comments/{comment_id}"), &Body { body }).await
}

pub async fn delete_comment(comment_id: String) -> bool {
    delete_ok(&format!("/comments/{comment_id}")).await
}

pub async fn vote_comment(comment_id: String, value: i32) -> bool {
    #[derive(Serialize)]
    struct Body {
        value: i32,
    }
    req(Method::POST, &format!("/comments/{comment_id}/vote"))
        .json(&Body { value })
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── Translations ──────────────────────────────────────────────────────────────

/// Looks up translations with scope priority: chapter → book → global.
pub async fn fetch_word_translations(
    word: String,
    target_lang: String,
    book_slug: Option<String>,
    chapter_slug: Option<String>,
) -> Option<Vec<WordTranslation>> {
    let path = match (&book_slug, &chapter_slug) {
        (Some(b), Some(c)) => format!("/books/{b}/chapters/{c}/translations"),
        (Some(b), None) => format!("/books/{b}/translations"),
        _ => "/translations".to_string(),
    };
    Client::new()
        .get(url(&path))
        .query(&[("word", &word), ("lang", &target_lang)])
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()
}

pub async fn submit_translation(input: CreateTranslationInput) -> Option<WordTranslation> {
    post_json("/translations", &input).await
}

pub async fn vote_translation(translation_id: String, value: i32) -> bool {
    #[derive(Serialize)]
    struct Body {
        value: i32,
    }
    req(Method::POST, &format!("/translations/{translation_id}/vote"))
        .json(&Body { value })
        .send()
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── Collections ───────────────────────────────────────────────────────────────

pub async fn fetch_my_collections() -> Option<Vec<Collection>> {
    get_json("/me/collections").await
}

pub async fn create_collection(input: CreateCollectionInput) -> Option<Collection> {
    post_json("/me/collections", &input).await
}

pub async fn fetch_collection(collection_id: String) -> Option<Collection> {
    get_json(&format!("/me/collections/{collection_id}")).await
}

pub async fn update_collection(
    collection_id: String,
    input: UpdateCollectionInput,
) -> Option<Collection> {
    put_json(&format!("/me/collections/{collection_id}"), &input).await
}

pub async fn delete_collection(collection_id: String) -> bool {
    delete_ok(&format!("/me/collections/{collection_id}")).await
}

pub async fn add_book_to_collection(
    collection_id: String,
    input: AddBookToCollectionInput,
) -> Option<CollectionBook> {
    post_json(&format!("/me/collections/{collection_id}/books"), &input).await
}

pub async fn remove_book_from_collection(collection_id: String, book_slug: String) -> bool {
    delete_ok(&format!("/me/collections/{collection_id}/books/{book_slug}")).await
}

// ── Bookmarks ─────────────────────────────────────────────────────────────────

pub async fn fetch_my_bookmarks(status: Option<String>) -> Option<Vec<Bookmark>> {
    let mut r = Client::new().get(url("/me/bookmarks"));
    if let Some(s) = status {
        r = r.query(&[("status", s)]);
    }
    r.send().await.ok()?.json().await.ok()
}

pub async fn upsert_bookmark(book_slug: String, input: UpsertBookmarkInput) -> Option<Bookmark> {
    put_json(&format!("/books/{book_slug}/bookmark"), &input).await
}

pub async fn delete_bookmark(book_slug: String) -> bool {
    delete_ok(&format!("/books/{book_slug}/bookmark")).await
}

// ── Reading goal ──────────────────────────────────────────────────────────────

pub async fn fetch_my_reading_goal() -> Option<ReadingGoal> {
    get_json("/me/reading-goal").await
}

pub async fn upsert_reading_goal(input: UpsertReadingGoalInput) -> Option<ReadingGoal> {
    put_json("/me/reading-goal", &input).await
}
