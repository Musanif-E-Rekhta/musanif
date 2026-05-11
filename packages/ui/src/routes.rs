use dioxus::prelude::*;

use crate::components::ToastViewport;
use crate::mobile_shell::MobileShell;
use crate::navbar::AppNavbar;
use crate::views::{
    author_detail::AuthorDetail, authors::Authors, book_detail::BookDetail,
    chapter_reader::ChapterReader, home::Home, login::Login, login_2fa::Login2fa,
    not_found::NotFound, profile::Profile, settings::Settings, setup_2fa::Setup2fa, shelf::Shelf,
    signup::Signup,
};

#[component]
fn AppLayout() -> Element {
    rsx! {
        if cfg!(feature = "mobile") {
            MobileShell {}
        } else {
            AppNavbar {}
        }
        ToastViewport {}
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
        #[route("/settings/2fa")]
        Setup2fa {},
    #[end_layout]
    // Auth routes outside the sidebar layout
    #[route("/login")]
    Login {},
    #[route("/login/2fa")]
    Login2fa {},
    #[route("/signup")]
    Signup {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

impl Route {
    /// Page title shown in chrome (navbar, desktop breadcrumb).
    /// Empty string means "no title" — current chrome hides the segment.
    pub fn title(&self) -> &'static str {
        match self {
            Route::Home {} => "Discover",
            Route::Shelf {} => "My Shelf",
            Route::Authors {} | Route::AuthorDetail { .. } => "Authors",
            Route::Profile {} => "Profile",
            Route::Settings {} | Route::Setup2fa {} => "Settings",
            Route::BookDetail { .. } | Route::ChapterReader { .. } => "Reading",
            Route::Login {} | Route::Login2fa {} | Route::Signup {} | Route::NotFound { .. } => "",
        }
    }
}
