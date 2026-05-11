use dioxus::prelude::*;

use crate::components::{DiscoverRail, NavIsland, RailIsland, ThemeSwitcher};
use crate::state::{
    CURRENT_BOOK_SLUG, CURRENT_PAGE_TITLE, NAVIGATE_HOME, NAVIGATE_TO_BOOK_SLUG,
};
use crate::{theme, Route};

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

enum ShellLayout {
    Focus,
    Discover,
    Two,
}

fn layout_for(route: &Route) -> ShellLayout {
    match route {
        Route::ChapterReader { .. } => ShellLayout::Focus,
        Route::Home {} => ShellLayout::Discover,
        _ => ShellLayout::Two,
    }
}

/// Router layout: grid shell with island sidebar + scrollable main content.
///
/// `Discover` uses a 3-column layout with a live continue-reading rail on
/// the right; the chapter reader collapses to a focus-mode icon rail;
/// everything else uses the wide Slack-style sidebar.
#[component]
pub fn AppNavbar() -> Element {
    let nav = use_navigator();
    let current_route = use_route::<Route>();

    use_effect(move || {
        if *NAVIGATE_HOME.read() {
            *NAVIGATE_HOME.write() = false;
            nav.push(Route::Home {});
        }
        if let Some(slug) = NAVIGATE_TO_BOOK_SLUG.read().clone() {
            *NAVIGATE_TO_BOOK_SLUG.write() = None;
            nav.push(Route::BookDetail { slug });
        }
    });

    use_effect(theme::load_persisted);

    *CURRENT_PAGE_TITLE.write() = current_route.title();
    *CURRENT_BOOK_SLUG.write() = match &current_route {
        Route::BookDetail { slug } => Some(slug.clone()),
        Route::ChapterReader { book_slug, .. } => Some(book_slug.clone()),
        _ => None,
    };

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        match layout_for(&current_route) {
            ShellLayout::Focus => rsx! {
                div { class: "is-shell is-shell--focus",
                    RailIsland {}
                    Outlet::<Route> {}
                }
            },
            ShellLayout::Discover => rsx! {
                div { class: "is-shell",
                    NavIsland {}
                    Outlet::<Route> {}
                    DiscoverRail {}
                }
            },
            ShellLayout::Two => rsx! {
                div { class: "is-shell is-shell--two",
                    NavIsland {}
                    Outlet::<Route> {}
                }
            },
        }

        ThemeSwitcher {}
    }
}

/// Legacy slot-based navbar for platform wrappers.
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div { id: "navbar", {children} }
    }
}
