use dioxus::prelude::*;

use crate::mobile_shell::MobileShell;
use crate::navbar::AppNavbar;
use crate::views::{
    author_detail::AuthorDetail, authors::Authors, book_detail::BookDetail,
    chapter_reader::ChapterReader, home::Home, login::Login, not_found::NotFound, profile::Profile,
    settings::Settings, shelf::Shelf, signup::Signup,
};

#[component]
fn AppLayout() -> Element {
    if cfg!(feature = "mobile") {
        rsx! { MobileShell {} }
    } else {
        rsx! { AppNavbar {} }
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        Home {},
        #[route("/books/:slug")]
        BookDetail { slug: String },
        #[route("/books/:book_slug/chapters/:chapter_slug")]
        ChapterReader { book_slug: String, chapter_slug: String },
        #[route("/authors")]
        Authors {},
        #[route("/authors/:slug")]
        AuthorDetail { slug: String },
        #[route("/shelf")]
        Shelf {},
        #[route("/profile")]
        Profile {},
        #[route("/settings")]
        Settings {},
    #[end_layout]
    // Auth routes outside the sidebar layout
    #[route("/login")]
    Login {},
    #[route("/signup")]
    Signup {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
