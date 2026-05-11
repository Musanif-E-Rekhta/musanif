//! Global application state — signals shared across views and platform shells.
//!
//! Co-locating these here keeps lifecycle and ownership obvious instead of
//! scattering globals across `lib.rs`, `navbar.rs`, and view modules.

use dioxus::prelude::*;

use crate::{models::User, theme::Theme};

pub static CURRENT_THEME: GlobalSignal<Theme> = Signal::global(|| Theme::Parchment);
pub static CURRENT_PAGE_TITLE: GlobalSignal<&'static str> = Signal::global(|| "Discover");
pub static CURRENT_USER: GlobalSignal<Option<User>> = Signal::global(|| None);

/// Set to `true` to request a navigation to `Route::Home`.
/// The navbar's effect consumes the flag and resets it.
pub static NAVIGATE_HOME: GlobalSignal<bool> = Signal::global(|| false);

/// Slug of the book the current route is viewing (book detail or chapter
/// reader). Used by the desktop tab strip to highlight the active tab,
/// since the chrome lives outside the `Router`.
pub static CURRENT_BOOK_SLUG: GlobalSignal<Option<String>> = Signal::global(|| None);

/// Setting this to `Some(slug)` asks the navbar to push `Route::BookDetail`.
/// Used by the desktop tab strip to navigate from outside the `Router`.
pub static NAVIGATE_TO_BOOK_SLUG: GlobalSignal<Option<String>> = Signal::global(|| None);

/// Pending 2FA challenge token, set by the Login view when the server
/// responds with `requires_2fa = true`. The Login2fa view reads this
/// and clears it after a successful exchange.
pub static CURRENT_2FA_CHALLENGE: GlobalSignal<Option<String>> = Signal::global(|| None);
