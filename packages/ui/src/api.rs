use crate::models::*;

pub const API_BASE: &str = "http://localhost:3000/api/v1";

pub async fn fetch_books(q: Option<String>, lang: Option<String>) -> Option<Vec<Book>> {
    let client = reqwest::Client::new();
    let mut req = client.get(format!("{API_BASE}/books"));
    if let Some(q) = q {
        req = req.query(&[("q", q)]);
    }
    if let Some(lang) = lang {
        req = req.query(&[("lang", lang)]);
    }
    req.send().await.ok()?.json::<Vec<Book>>().await.ok()
}

pub async fn fetch_book(slug: String) -> Option<Book> {
    reqwest::get(format!("{API_BASE}/books/{slug}"))
        .await
        .ok()?
        .json::<Book>()
        .await
        .ok()
}

pub async fn fetch_chapters(book_slug: String) -> Option<Vec<ChapterSummary>> {
    reqwest::get(format!("{API_BASE}/books/{book_slug}/chapters"))
        .await
        .ok()?
        .json::<Vec<ChapterSummary>>()
        .await
        .ok()
}

pub async fn fetch_chapter(book_slug: String, chapter_slug: String) -> Option<Chapter> {
    reqwest::get(format!(
        "{API_BASE}/books/{book_slug}/chapters/{chapter_slug}"
    ))
    .await
    .ok()?
    .json::<Chapter>()
    .await
    .ok()
}

pub async fn fetch_author(slug: String) -> Option<Author> {
    reqwest::get(format!("{API_BASE}/authors/{slug}"))
        .await
        .ok()?
        .json::<Author>()
        .await
        .ok()
}

pub async fn fetch_author_books(slug: String) -> Option<Vec<Book>> {
    reqwest::get(format!("{API_BASE}/authors/{slug}/books"))
        .await
        .ok()?
        .json::<Vec<Book>>()
        .await
        .ok()
}

pub async fn fetch_book_reviews(book_slug: String) -> Option<Vec<BookReview>> {
    reqwest::get(format!(
        "{API_BASE}/books/{book_slug}/reviews?spoilers=false&limit=10"
    ))
    .await
    .ok()?
    .json::<Vec<BookReview>>()
    .await
    .ok()
}

pub async fn fetch_my_bookmarks(status: Option<String>) -> Option<Vec<Bookmark>> {
    let client = reqwest::Client::new();
    let mut req = client.get(format!("{API_BASE}/me/bookmarks"));
    if let Some(status) = status {
        req = req.query(&[("status", status)]);
    }
    req.send().await.ok()?.json::<Vec<Bookmark>>().await.ok()
}
