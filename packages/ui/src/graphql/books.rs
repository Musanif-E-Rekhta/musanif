//! Catalogue queries: books, authors, chapters, reviews, translations.

use cynic::QueryBuilder;

use super::client::run;
use super::schema::schema;
use crate::models;

// ── Cynic types ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct BookGql {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub isbn: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub page_count: Option<i32>,
    pub language: String,
    pub avg_rating: Option<f64>,
    pub review_count: i32,
    pub chapter_count: i32,
    pub is_published: bool,
    pub authors: Vec<BookAuthorEdgeGql>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct BookAuthorEdgeGql {
    pub author: BookAuthorGql,
    pub role: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct FeaturedBookGql {
    pub book: BookGql,
    pub featured_until: Option<String>,
    pub eyebrow: Option<String>,
    pub headline: Option<String>,
    pub blurb: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct BookAuthorGql {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website: Option<String>,
    pub is_following: bool,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ChapterListItemGql {
    pub id: String,
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
    pub summary: Option<String>,
    pub reading_time_mins: Option<i32>,
    pub avg_rating: Option<f64>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ChapterNavGql {
    pub number: i32,
    pub title: Option<String>,
    pub slug: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
pub struct ChapterGql {
    pub id: String,
    pub book_id: String,
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
    pub review_count: i32,
    pub is_published: bool,
    pub prev_chapter: Option<ChapterNavGql>,
    pub next_chapter: Option<ChapterNavGql>,
}

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
pub struct WordTranslationGql {
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
}

// ── Variables ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct BooksVars {
    pub q: Option<String>,
    pub lang: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SlugVars {
    pub slug: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AuthorsVars {
    pub q: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct BooksByAuthorVars {
    pub author_slug: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct BookSlugVars {
    pub book_slug: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ChapterVars {
    pub book_slug: String,
    pub chapter_slug: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct BookReviewsVars {
    pub book_slug: String,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub spoilers: Option<bool>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct WordTranslationsVars {
    pub word: String,
    pub target_lang: String,
    pub book_slug: Option<String>,
    pub chapter_slug: Option<String>,
}

// ── Queries ───────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "BooksVars")]
pub struct BooksQuery {
    #[arguments(q: $q, lang: $lang, limit: $limit, offset: $offset)]
    pub books: Vec<BookGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot")]
pub struct FeaturedQuery {
    pub featured: Option<FeaturedBookGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "SlugVars")]
pub struct BookQuery {
    #[arguments(slug: $slug)]
    pub book: Option<BookGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "AuthorsVars")]
pub struct AuthorsQuery {
    #[arguments(q: $q, limit: $limit, offset: $offset)]
    pub authors: Vec<BookAuthorGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "SlugVars")]
pub struct AuthorQuery {
    #[arguments(slug: $slug)]
    pub author: Option<BookAuthorGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "BooksByAuthorVars")]
pub struct BooksByAuthorQuery {
    #[arguments(authorSlug: $author_slug)]
    pub books_by_author: Vec<BookGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "BookSlugVars")]
pub struct ChaptersQuery {
    #[arguments(bookSlug: $book_slug)]
    pub chapters: Vec<ChapterListItemGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "ChapterVars")]
pub struct ChapterQuery {
    #[arguments(bookSlug: $book_slug, chapterSlug: $chapter_slug)]
    pub chapter: Option<ChapterGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "BookReviewsVars")]
pub struct BookReviewsQuery {
    #[arguments(bookSlug: $book_slug, limit: $limit, offset: $offset, spoilers: $spoilers)]
    pub book_reviews: Vec<BookReviewGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "WordTranslationsVars")]
pub struct WordTranslationsQuery {
    #[arguments(word: $word, targetLang: $target_lang, bookSlug: $book_slug, chapterSlug: $chapter_slug)]
    pub word_translations: Vec<WordTranslationGql>,
}

// ── Conversions ───────────────────────────────────────────────────────────────

impl From<BookGql> for models::Book {
    fn from(b: BookGql) -> Self {
        let authors = b.authors.into_iter().map(Into::into).collect::<Vec<_>>();
        models::Book {
            id: b.id,
            title: b.title,
            slug: b.slug,
            isbn: b.isbn,
            summary: b.summary,
            description: b.description,
            cover_url: b.cover_url,
            page_count: b.page_count,
            language: b.language,
            published_at: None,
            avg_rating: b.avg_rating,
            review_count: b.review_count,
            chapter_count: b.chapter_count,
            is_published: b.is_published,
            authors: Some(authors),
            categories: None,
            tags: None,
            created_at: None,
            updated_at: None,
        }
    }
}

impl From<BookAuthorEdgeGql> for models::BookAuthor {
    fn from(e: BookAuthorEdgeGql) -> Self {
        models::BookAuthor {
            author: models::Author::from(e.author),
            role: e.role,
        }
    }
}

impl From<FeaturedBookGql> for models::FeaturedBook {
    fn from(f: FeaturedBookGql) -> Self {
        models::FeaturedBook {
            book: f.book.into(),
            featured_until: f.featured_until,
            eyebrow: f.eyebrow,
            headline: f.headline,
            blurb: f.blurb,
        }
    }
}

impl From<BookAuthorGql> for models::Author {
    fn from(a: BookAuthorGql) -> Self {
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

impl From<ChapterListItemGql> for models::ChapterSummary {
    fn from(c: ChapterListItemGql) -> Self {
        models::ChapterSummary {
            id: c.id,
            number: c.number,
            title: c.title,
            slug: c.slug,
            reading_time_mins: c.reading_time_mins,
            avg_rating: c.avg_rating,
            is_published: true,
        }
    }
}

impl From<ChapterNavGql> for models::ChapterNav {
    fn from(c: ChapterNavGql) -> Self {
        models::ChapterNav {
            number: c.number,
            title: c.title,
            slug: c.slug,
        }
    }
}

impl From<ChapterGql> for models::Chapter {
    fn from(c: ChapterGql) -> Self {
        models::Chapter {
            id: c.id,
            number: c.number,
            title: c.title,
            slug: c.slug,
            content: c.content,
            content_format: c.content_format,
            summary: c.summary,
            meta_description: c.meta_description,
            word_count: c.word_count,
            reading_time_mins: c.reading_time_mins,
            avg_rating: c.avg_rating,
            review_count: c.review_count,
            is_published: c.is_published,
            published_at: None,
            updated_at: None,
            prev_chapter: c.prev_chapter.map(Into::into),
            next_chapter: c.next_chapter.map(Into::into),
            book: None,
        }
    }
}

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

impl From<WordTranslationGql> for models::WordTranslation {
    fn from(w: WordTranslationGql) -> Self {
        models::WordTranslation {
            id: w.id,
            word: w.word,
            translation: w.translation,
            source_lang: w.source_lang,
            target_lang: w.target_lang,
            submitted_by: w.submitted_by,
            scope: w.scope,
            book_id: w.book_id,
            chapter_id: w.chapter_id,
            context_note: w.context_note,
            upvotes: w.upvotes,
            downvotes: w.downvotes,
            score: w.score,
            created_at: String::new(),
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn fetch_books(
    q: Option<String>,
    lang: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<models::Book>> {
    let op = BooksQuery::build(BooksVars {
        q,
        lang,
        limit,
        offset,
    });
    run(op)
        .await
        .map(|d| d.books.into_iter().map(Into::into).collect())
}

pub async fn fetch_featured() -> Option<models::FeaturedBook> {
    let op = FeaturedQuery::build(());
    run(op).await.and_then(|d| d.featured.map(Into::into))
}

pub async fn fetch_book(slug: String) -> Option<models::Book> {
    let op = BookQuery::build(SlugVars { slug });
    run(op).await.and_then(|d| d.book.map(Into::into))
}

pub async fn fetch_authors(
    q: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<models::Author>> {
    let op = AuthorsQuery::build(AuthorsVars { q, limit, offset });
    run(op)
        .await
        .map(|d| d.authors.into_iter().map(Into::into).collect())
}

pub async fn fetch_author(slug: String) -> Option<models::Author> {
    let op = AuthorQuery::build(SlugVars { slug });
    run(op).await.and_then(|d| d.author.map(Into::into))
}

pub async fn fetch_books_by_author(
    slug: String,
    _limit: Option<i32>,
    _offset: Option<i32>,
) -> Option<Vec<models::Book>> {
    let op = BooksByAuthorQuery::build(BooksByAuthorVars { author_slug: slug });
    run(op)
        .await
        .map(|d| d.books_by_author.into_iter().map(Into::into).collect())
}

pub async fn fetch_chapters(book_slug: String) -> Option<Vec<models::ChapterSummary>> {
    let op = ChaptersQuery::build(BookSlugVars { book_slug });
    run(op)
        .await
        .map(|d| d.chapters.into_iter().map(Into::into).collect())
}

pub async fn fetch_chapter(
    book_slug: String,
    chapter_slug: String,
) -> Option<models::Chapter> {
    let op = ChapterQuery::build(ChapterVars {
        book_slug,
        chapter_slug,
    });
    run(op).await.and_then(|d| d.chapter.map(Into::into))
}

pub async fn fetch_book_reviews(
    book_slug: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<models::BookReview>> {
    let op = BookReviewsQuery::build(BookReviewsVars {
        book_slug,
        limit,
        offset,
        spoilers: None,
    });
    run(op)
        .await
        .map(|d| d.book_reviews.into_iter().map(Into::into).collect())
}

pub async fn fetch_word_translations(
    word: String,
    target_lang: String,
    book_slug: Option<String>,
    chapter_slug: Option<String>,
) -> Option<Vec<models::WordTranslation>> {
    let op = WordTranslationsQuery::build(WordTranslationsVars {
        word,
        target_lang,
        book_slug,
        chapter_slug,
    });
    run(op)
        .await
        .map(|d| d.word_translations.into_iter().map(Into::into).collect())
}
