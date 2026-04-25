/// GraphQL client.
///
/// The server is assumed to return snake_case field names
/// (async-graphql supports this via `rename_all = "snake_case"`),
/// so existing model types deserialise without extra aliases.
///
/// Usage:
///   let books = graphql::fetch_books(Some("tolkien".into()), None, None, None).await;
///   let payload = graphql::register("alice".into(), "a@b.com".into(), "pw".into()).await;
use serde::{Deserialize, Serialize};

use crate::{api::get_auth_token, config, models::*};

// ── Core types ────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct GqlRequest<V: Serialize> {
    query: String,
    variables: V,
}

#[derive(Deserialize)]
struct GqlResponse<T> {
    data: Option<T>,
    #[serde(default)]
    errors: Vec<GqlError>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GqlError {
    pub message: String,
}

// ── Executor ──────────────────────────────────────────────────────────────────

async fn execute<V, T>(query: &str, variables: V) -> Option<T>
where
    V: Serialize,
    T: for<'de> Deserialize<'de>,
{
    let mut req = reqwest::Client::new().post(config::graphql_url());
    if let Some(token) = get_auth_token() {
        req = req.bearer_auth(token);
    }
    let resp: GqlResponse<T> = req
        .json(&GqlRequest {
            query: query.to_string(),
            variables,
        })
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    // Surface errors in stderr / browser devtools without extra deps
    for e in &resp.errors {
        eprintln!("GQL error: {}", e.message);
    }
    resp.data
}

// ── Auth ──────────────────────────────────────────────────────────────────────

pub async fn register(username: String, email: String, password: String) -> Option<AuthPayload> {
    #[derive(Serialize)]
    struct Vars {
        username: String,
        email: String,
        password: String,
    }
    #[derive(Deserialize)]
    struct Data {
        register_user: AuthPayload,
    }
    execute::<_, Data>(
        r#"mutation Register($username: String!, $email: String!, $password: String!) {
            register_user(username: $username, email: $email, password: $password) {
                token
                user { id username email }
            }
        }"#,
        Vars {
            username,
            email,
            password,
        },
    )
    .await
    .map(|d| d.register_user)
}

pub async fn gql_login(email: String, password: String) -> Option<AuthPayload> {
    #[derive(Serialize)]
    struct Vars {
        email: String,
        password: String,
    }
    #[derive(Deserialize)]
    struct Data {
        login_user: AuthPayload,
    }
    execute::<_, Data>(
        r#"mutation Login($email: String!, $password: String!) {
            login_user(email: $email, password: $password) {
                token
                user { id username email avatar_url bio website location }
            }
        }"#,
        Vars { email, password },
    )
    .await
    .map(|d| d.login_user)
}

pub async fn forgot_password(email: String) -> bool {
    #[derive(Serialize)]
    struct Vars {
        email: String,
    }
    #[derive(Deserialize)]
    struct Data {
        forgot_password: bool,
    }
    execute::<_, Data>(
        r#"mutation ForgotPassword($email: String!) {
            forgot_password(email: $email)
        }"#,
        Vars { email },
    )
    .await
    .map(|d| d.forgot_password)
    .unwrap_or(false)
}

pub async fn reset_password_with_token(token: String, new_password: String) -> bool {
    #[derive(Serialize)]
    struct Vars {
        token: String,
        new_password: String,
    }
    #[derive(Deserialize)]
    struct Data {
        reset_password_with_token: bool,
    }
    execute::<_, Data>(
        r#"mutation ResetPassword($token: String!, $new_password: String!) {
            reset_password_with_token(token: $token, new_password: $new_password)
        }"#,
        Vars {
            token,
            new_password,
        },
    )
    .await
    .map(|d| d.reset_password_with_token)
    .unwrap_or(false)
}

// ── Me ────────────────────────────────────────────────────────────────────────

pub async fn fetch_me() -> Option<User> {
    #[derive(Deserialize)]
    struct Data {
        me: User,
    }
    execute::<(), Data>(
        r#"query Me {
            me { id username email avatar_url bio website location }
        }"#,
        (),
    )
    .await
    .map(|d| d.me)
}

pub async fn update_profile(input: UpdateProfileInput) -> Option<User> {
    #[derive(Serialize)]
    struct Vars {
        input: UpdateProfileInput,
    }
    #[derive(Deserialize)]
    struct Data {
        update_profile: User,
    }
    execute::<_, Data>(
        r#"mutation UpdateProfile($input: UpdateProfileInput!) {
            update_profile(input: $input) { id username email avatar_url bio website location }
        }"#,
        Vars { input },
    )
    .await
    .map(|d| d.update_profile)
}

pub async fn change_password(input: ChangePasswordInput) -> bool {
    #[derive(Serialize)]
    struct Vars {
        input: ChangePasswordInput,
    }
    #[derive(Deserialize)]
    struct Data {
        change_password: bool,
    }
    execute::<_, Data>(
        r#"mutation ChangePassword($input: ChangePasswordInput!) {
            change_password(input: $input)
        }"#,
        Vars { input },
    )
    .await
    .map(|d| d.change_password)
    .unwrap_or(false)
}

pub async fn delete_me() -> bool {
    #[derive(Deserialize)]
    struct Data {
        delete_me: bool,
    }
    execute::<(), Data>(
        r#"mutation DeleteMe {
            delete_me
        }"#,
        (),
    )
    .await
    .map(|d| d.delete_me)
    .unwrap_or(false)
}

pub async fn fetch_my_stats() -> Option<UserStats> {
    #[derive(Deserialize)]
    struct Data {
        my_stats: UserStats,
    }
    execute::<(), Data>(
        r#"query MyStats {
            my_stats { bookmark_counts highlight_count review_count session_count }
        }"#,
        (),
    )
    .await
    .map(|d| d.my_stats)
}

pub async fn fetch_my_reading_sessions() -> Option<Vec<ReadingSessionResponse>> {
    #[derive(Deserialize)]
    struct Data {
        my_reading_sessions: Vec<ReadingSessionResponse>,
    }
    execute::<(), Data>(
        r#"query MyReadingSessions {
            my_reading_sessions { id book_id book_title chapter_id chapter_number duration_mins occurred_at }
        }"#,
        (),
    )
    .await
    .map(|d| d.my_reading_sessions)
}

pub async fn fetch_my_following() -> Option<Vec<Author>> {
    #[derive(Deserialize)]
    struct Data {
        my_following: Vec<Author>,
    }
    execute::<(), Data>(
        r#"query MyFollowing {
            my_following { id name slug bio avatar_url website followers }
        }"#,
        (),
    )
    .await
    .map(|d| d.my_following)
}

// ── Books ─────────────────────────────────────────────────────────────────────

pub async fn fetch_books(
    q: Option<String>,
    lang: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<Book>> {
    #[derive(Serialize)]
    struct Vars {
        q: Option<String>,
        lang: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        books: Vec<Book>,
    }
    execute::<_, Data>(
        r#"query Books($q: String, $lang: String, $limit: Int, $offset: Int) {
            books(q: $q, lang: $lang, limit: $limit, offset: $offset) {
                id title slug summary cover_url language
                avg_rating review_count chapter_count is_published
            }
        }"#,
        Vars {
            q,
            lang,
            limit,
            offset,
        },
    )
    .await
    .map(|d| d.books)
}

pub async fn fetch_book(slug: String) -> Option<Book> {
    #[derive(Serialize)]
    struct Vars {
        slug: String,
    }
    #[derive(Deserialize)]
    struct Data {
        book: Book,
    }
    execute::<_, Data>(
        r#"query Book($slug: String!) {
            book(slug: $slug) {
                id title slug isbn summary description cover_url page_count
                language published_at avg_rating review_count chapter_count is_published
            }
        }"#,
        Vars { slug },
    )
    .await
    .map(|d| d.book)
}

// ── Authors ───────────────────────────────────────────────────────────────────

pub async fn fetch_authors(
    q: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<Author>> {
    #[derive(Serialize)]
    struct Vars {
        q: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        authors: Vec<Author>,
    }
    execute::<_, Data>(
        r#"query Authors($q: String, $limit: Int, $offset: Int) {
            authors(q: $q, limit: $limit, offset: $offset) {
                id name slug bio avatar_url website
            }
        }"#,
        Vars { q, limit, offset },
    )
    .await
    .map(|d| d.authors)
}

pub async fn fetch_books_by_author(
    slug: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<Book>> {
    #[derive(Serialize)]
    struct Vars {
        slug: String,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        books_by_author: Vec<Book>,
    }
    execute::<_, Data>(
        r#"query BooksByAuthor($slug: String!, $limit: Int, $offset: Int) {
            books_by_author(slug: $slug, limit: $limit, offset: $offset) {
                id title slug avg_rating is_published cover_url
            }
        }"#,
        Vars {
            slug,
            limit,
            offset,
        },
    )
    .await
    .map(|d| d.books_by_author)
}

// ── Chapters ──────────────────────────────────────────────────────────────────

pub async fn fetch_chapter(book_slug: String, chapter_slug: String) -> Option<Chapter> {
    #[derive(Serialize)]
    struct Vars {
        book_slug: String,
        chapter_slug: String,
    }
    #[derive(Deserialize)]
    struct Data {
        chapter: Chapter,
    }
    execute::<_, Data>(
        r#"query Chapter($book_slug: String!, $chapter_slug: String!) {
            chapter(book_slug: $book_slug, chapter_slug: $chapter_slug) {
                id number title slug content content_format summary
                word_count reading_time_mins avg_rating review_count
                prev_chapter { number title slug }
                next_chapter { number title slug }
            }
        }"#,
        Vars {
            book_slug,
            chapter_slug,
        },
    )
    .await
    .map(|d| d.chapter)
}

// ── Reviews ───────────────────────────────────────────────────────────────────

pub async fn fetch_book_reviews(
    book_slug: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<BookReview>> {
    #[derive(Serialize)]
    struct Vars {
        book_slug: String,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        book_reviews: Vec<BookReview>,
    }
    execute::<_, Data>(
        r#"query BookReviews($book_slug: String!, $limit: Int, $offset: Int) {
            book_reviews(book_slug: $book_slug, limit: $limit, offset: $offset) {
                id user_id rating title body contains_spoiler
                reading_status verified_reader helpful_count status created_at
            }
        }"#,
        Vars {
            book_slug,
            limit,
            offset,
        },
    )
    .await
    .map(|d| d.book_reviews)
}

// ── Highlights ────────────────────────────────────────────────────────────────

pub async fn fetch_my_highlights(
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<Highlight>> {
    #[derive(Serialize)]
    struct Vars {
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        my_highlights: Vec<Highlight>,
    }
    execute::<_, Data>(
        r#"query MyHighlights($limit: Int, $offset: Int) {
            my_highlights(limit: $limit, offset: $offset) {
                id chapter_id book_id offset_start offset_end
                text_snapshot color note is_public created_at
            }
        }"#,
        Vars { limit, offset },
    )
    .await
    .map(|d| d.my_highlights)
}

// ── Translations ──────────────────────────────────────────────────────────────

pub async fn fetch_word_translations(
    word: String,
    target_lang: String,
    book_slug: Option<String>,
    chapter_slug: Option<String>,
) -> Option<Vec<WordTranslation>> {
    #[derive(Serialize)]
    struct Vars {
        word: String,
        target_lang: String,
        book_slug: Option<String>,
        chapter_slug: Option<String>,
    }
    #[derive(Deserialize)]
    struct Data {
        word_translations: Vec<WordTranslation>,
    }
    execute::<_, Data>(
        r#"query WordTranslations(
            $word: String!, $target_lang: String!,
            $book_slug: String, $chapter_slug: String
        ) {
            word_translations(
                word: $word, target_lang: $target_lang,
                book_slug: $book_slug, chapter_slug: $chapter_slug
            ) {
                id word translation source_lang target_lang
                scope context_note upvotes downvotes score
            }
        }"#,
        Vars {
            word,
            target_lang,
            book_slug,
            chapter_slug,
        },
    )
    .await
    .map(|d| d.word_translations)
}

// ── Shelf ─────────────────────────────────────────────────────────────────────

pub async fn fetch_my_shelf(
    status: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<Bookmark>> {
    #[derive(Serialize)]
    struct Vars {
        status: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        my_bookmarks: Vec<Bookmark>,
    }
    execute::<_, Data>(
        r#"query MyShelf($status: String, $limit: Int, $offset: Int) {
            my_bookmarks(status: $status, limit: $limit, offset: $offset) {
                id book_id status progress notes
            }
        }"#,
        Vars {
            status,
            limit,
            offset,
        },
    )
    .await
    .map(|d| d.my_bookmarks)
}

pub async fn fetch_reading_goal(year: Option<i32>) -> Option<ReadingGoal> {
    #[derive(Serialize)]
    struct Vars {
        year: Option<i32>,
    }
    #[derive(Deserialize)]
    struct Data {
        my_reading_goal: ReadingGoal,
    }
    execute::<_, Data>(
        r#"query ReadingGoal($year: Int) {
            my_reading_goal(year: $year) {
                id year target completed progress_pct
            }
        }"#,
        Vars { year },
    )
    .await
    .map(|d| d.my_reading_goal)
}

// ── Mutations ─────────────────────────────────────────────────────────────────

pub async fn upsert_bookmark(
    book_slug: String,
    status: String,
    progress: Option<i32>,
    notes: Option<String>,
) -> Option<Bookmark> {
    #[derive(Serialize)]
    struct Input {
        book_slug: String,
        status: String,
        progress: Option<i32>,
        notes: Option<String>,
    }
    #[derive(Serialize)]
    struct Vars {
        input: Input,
    }
    #[derive(Deserialize)]
    struct Data {
        upsert_bookmark: Bookmark,
    }
    execute::<_, Data>(
        r#"mutation UpsertBookmark($input: UpsertBookmarkInput!) {
            upsert_bookmark(input: $input) { id book_id status progress }
        }"#,
        Vars {
            input: Input {
                book_slug,
                status,
                progress,
                notes,
            },
        },
    )
    .await
    .map(|d| d.upsert_bookmark)
}

pub async fn submit_book_review(
    book_slug: String,
    rating: i32,
    title: Option<String>,
    body: Option<String>,
    contains_spoiler: bool,
    reading_status: String,
) -> Option<BookReview> {
    #[derive(Serialize)]
    struct Input {
        book_slug: String,
        rating: i32,
        title: Option<String>,
        body: Option<String>,
        contains_spoiler: bool,
        reading_status: String,
    }
    #[derive(Serialize)]
    struct Vars {
        input: Input,
    }
    #[derive(Deserialize)]
    struct Data {
        create_book_review: BookReview,
    }
    execute::<_, Data>(
        r#"mutation ReviewBook($input: CreateBookReviewInput!) {
            create_book_review(input: $input) {
                id rating title body verified_reader helpful_count status
            }
        }"#,
        Vars {
            input: Input {
                book_slug,
                rating,
                title,
                body,
                contains_spoiler,
                reading_status,
            },
        },
    )
    .await
    .map(|d| d.create_book_review)
}

pub async fn upsert_reading_goal(year: i32, target: i32) -> Option<ReadingGoal> {
    #[derive(Serialize)]
    struct Vars {
        year: i32,
        target: i32,
    }
    #[derive(Deserialize)]
    struct Data {
        upsert_reading_goal: ReadingGoal,
    }
    execute::<_, Data>(
        r#"mutation SetGoal($year: Int!, $target: Int!) {
            upsert_reading_goal(year: $year, target: $target) {
                year target completed progress_pct
            }
        }"#,
        Vars { year, target },
    )
    .await
    .map(|d| d.upsert_reading_goal)
}
