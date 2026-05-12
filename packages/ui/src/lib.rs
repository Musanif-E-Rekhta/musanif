//! Shared UI library for every Musanif platform shell (web, desktop,
//! mobile, admin).
//!
//! Modules:
//! - [`api`] — public API surface consumed by views; thin re-exports of
//!   [`graphql`] plus the multipart [`api::uploads`] / binary
//!   [`api::binary`] paths that don't fit GraphQL.
//! - [`graphql`] — cynic-typed operations against
//!   `musanif-contracts/schema.graphql`, with thin wrappers that
//!   project responses into [`models`] types.
//! - [`components`] — design-system pieces (`Cover`, `FeatureCard`,
//!   `AuthField`, `Toggle`, …) used by views.
//! - [`views`] — one component per route in [`Route`].
//! - [`models`] — domain types consumed by views (`Book`, `Chapter`,
//!   `User`, `UserMe`, `Profile`, `Highlight`, …). Cynic responses are
//!   converted into these so view code never touches cynic directly.
//! - [`prefs`] — `localStorage`-backed persistence for display, reading,
//!   and library toggles (no backend mutation yet).
//! - [`theme`] — applies `data-theme` to `<html>` and persists choice.
//! - [`state`] — global signals (`CURRENT_USER`, `CURRENT_THEME`, …).
//! - [`mobile_shell`] — outer shell rendered when the `mobile` feature
//!   is on.
//!
//! Platform shells should mount [`Route`] in their entry point and
//! contribute platform-specific chrome (window decorations, mobile
//! bottom nav, admin sidebar) on top.

// `macros` must come before any module that uses `from_as_error!` —
// `#[macro_export]` puts the macro at the crate root, but its body
// references `$crate::error::Error`, so the order keeps the resolver
// happy on first compile.
#[macro_use]
mod macros;

pub mod api;
pub mod components;
pub mod config;
pub mod curation;
pub mod error;
pub mod graphql;
pub mod mobile_shell;
pub mod models;
pub mod prefs;
pub mod state;
pub mod theme;
pub mod views;

mod navbar;
mod routes;

pub use error::{Error, Result};

pub use mobile_shell::MobileShell;
pub use navbar::{AppNavbar, Navbar};
pub use routes::Route;
pub use state::{
    CURRENT_BOOK_SLUG, CURRENT_PAGE_TITLE, CURRENT_THEME, CURRENT_USER, NAVIGATE_HOME,
    NAVIGATE_TO_BOOK_SLUG,
};
pub use theme::Theme;
