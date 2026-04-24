use serde::{Deserialize, Serialize};

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
pub struct ReadingGoal {
    pub id: String,
    pub year: i32,
    pub target: i32,
    pub completed: i32,
    pub progress_pct: f64,
}
