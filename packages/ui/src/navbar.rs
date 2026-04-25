use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdSearch, LdSettings, LdUser},
    Icon,
};

use crate::Route;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

pub static NAVIGATE_HOME: GlobalSignal<bool> = Signal::global(|| false);

/// Router layout: grid shell with island sidebar + scrollable main content.
/// Home uses 3-column (nav | main | rail); all other routes use 2-column.
#[component]
pub fn AppNavbar() -> Element {
    let nav = use_navigator();
    let current_route = use_route::<Route>();

    use_effect(move || {
        if *NAVIGATE_HOME.read() {
            *NAVIGATE_HOME.write() = false;
            nav.push(Route::Home {});
        }
    });

    // Apply saved theme on mount
    use_effect(move || {
        let _ = document::eval(
            r#"
            try {
              const t = localStorage.getItem('musanif-theme') || 'parchment';
              document.documentElement.setAttribute('data-theme', t);
            } catch(e) {}
            "#,
        );
    });

    let shell_class = "is-shell is-shell--two";

    let page_title: &'static str = match &current_route {
        Route::Home {} => "Discover",
        Route::Shelf {} => "My Shelf",
        Route::Authors {} | Route::AuthorDetail { .. } => "Authors",
        Route::Profile {} => "Profile",
        Route::Settings {} => "Settings",
        Route::BookDetail { .. } | Route::ChapterReader { .. } => "Reading",
        _ => "",
    };
    *crate::CURRENT_PAGE_TITLE.write() = page_title;

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        div { class: "{shell_class}",
            // Column 1: Navigation island
            aside { class: "island is-nav",
                div { class: "is-nav-brand",
                    span { class: "is-nav-brand-mark", "مصنف" }
                    span { class: "is-nav-brand-name", "Musanif" }
                }

                Link {
                    to: Route::Home {},
                    class: "is-nav-item",
                    active_class: "is-nav-item--active",
                    Icon { icon: LdCompass, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Discover"
                }
                Link {
                    to: Route::Shelf {},
                    class: "is-nav-item",
                    active_class: "is-nav-item--active",
                    Icon { icon: LdLibrary, width: 16, height: 16, class: "is-nav-item-icon" }
                    "My Shelf"
                }
                Link {
                    to: Route::Authors {},
                    class: "is-nav-item",
                    active_class: "is-nav-item--active",
                    Icon { icon: LdPenLine, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Authors"
                }
                Link {
                    to: Route::Home {},
                    class: "is-nav-item",
                    Icon { icon: LdSearch, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Search"
                }


                div { class: "is-nav-spacer" }

                Link {
                    to: Route::Settings {},
                    class: "is-nav-item",
                    active_class: "is-nav-item--active",
                    Icon { icon: LdSettings, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Settings"
                }

                Link {
                    to: Route::Profile {},
                    class: "is-nav-user",
                    active_class: "is-nav-user--active",
                    div { class: "is-nav-user-avatar",
                        if let Some(user) = crate::CURRENT_USER.read().as_ref() {
                            {user.username.chars().next().unwrap_or('?').to_ascii_uppercase().to_string()}
                        } else {
                            Icon { icon: LdUser, width: 16, height: 16 }
                        }
                    }
                    div { style: "flex: 1",
                        if let Some(user) = crate::CURRENT_USER.read().as_ref() {
                            div { class: "is-nav-user-name", "{user.username}" }
                            div { class: "is-nav-user-meta", "View profile" }
                        } else {
                            div { class: "is-nav-user-name", "Guest" }
                            div { class: "is-nav-user-meta", "Not signed in" }
                        }
                    }
                }
            }

            // Column 2: Main content (each view provides its own island is-main)
            Outlet::<Route> {}
        }

        crate::components::ThemeSwitcher {}
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
