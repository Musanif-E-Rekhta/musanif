use dioxus::prelude::*;

use crate::navbar::AppNavbar;
use crate::views::{
    author_detail::AuthorDetail, book_detail::BookDetail, chapter_reader::ChapterReader,
    home::Home, not_found::NotFound, shelf::Shelf,
};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppNavbar)]
        #[route("/")]
        Home {},
        #[route("/books/:slug")]
        BookDetail { slug: String },
        #[route("/books/:book_slug/chapters/:chapter_slug")]
        ChapterReader { book_slug: String, chapter_slug: String },
        #[route("/authors/:slug")]
        AuthorDetail { slug: String },
        #[route("/shelf")]
        Shelf {},
    #[end_layout]
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
