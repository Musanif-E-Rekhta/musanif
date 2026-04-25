//! Shared UI library: models, API client, GraphQL client, routes, and views.

pub mod api;
pub mod components;
pub mod config;
pub mod graphql;
pub mod mobile_shell;
pub mod models;
pub mod views;

mod navbar;
mod routes;

use dioxus::prelude::*;
pub use mobile_shell::MobileShell;
pub use navbar::{AppNavbar, Navbar, NAVIGATE_HOME};
pub use routes::Route;

#[derive(Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum Theme {
    #[default]
    Parchment,
    Midnight,
    SepiaDark,
    Ink,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Parchment => "parchment",
            Theme::Midnight => "midnight",
            Theme::SepiaDark => "sepia-dark",
            Theme::Ink => "ink",
        }
    }
}

pub static CURRENT_THEME: GlobalSignal<Theme> = Signal::global(|| Theme::Parchment);
pub static CURRENT_PAGE_TITLE: GlobalSignal<&'static str> = Signal::global(|| "Discover");
