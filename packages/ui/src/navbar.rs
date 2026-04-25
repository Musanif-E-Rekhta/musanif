use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{
        LdBookOpen, LdCompass, LdHighlighter, LdLibrary, LdPenLine, LdSearch, LdSettings, LdUser,
    },
    Icon,
};

use crate::{components::Cover, Route};

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

    let is_home = matches!(current_route, Route::Home {});
    let shell_class = if is_home {
        "is-shell"
    } else {
        "is-shell is-shell--two"
    };

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
                    span { class: "is-nav-item-count", "12" }
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

                div { class: "is-nav-section", "Reading" }
                button { class: "is-nav-item",
                    Icon { icon: LdBookOpen, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Continue · Ch.47"
                }
                button { class: "is-nav-item",
                    Icon { icon: LdHighlighter, width: 16, height: 16, class: "is-nav-item-icon" }
                    "Highlights"
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
                        Icon { icon: LdUser, width: 16, height: 16 }
                    }
                    div { style: "flex: 1",
                        div { class: "is-nav-user-name", "Usaira Imisani" }
                        div { class: "is-nav-user-meta", "Reader · 142 books" }
                    }
                }
            }

            // Column 2: Main content (each view provides its own island is-main)
            Outlet::<Route> {}

            // Column 3: Right rail (only on Home — grid slot is absent on is-shell--two)
            if is_home {
                ContinueRail {}
            }
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

#[component]
fn ContinueRail() -> Element {
    rsx! {
        div { class: "is-rail-col", style: "overflow-y: auto; padding: 0",
            div { class: "island is-rail-card",
                div { class: "is-rail-eyebrow", "Continue Reading" }
                div { class: "is-continue",
                    div { class: "is-continue-cover", style: "background: #e8d5c4",
                        Cover { urdu: "بانگِ درا", mono: "ا" }
                    }
                    div { class: "is-continue-info",
                        p { class: "is-continue-title", "Bang-e-Dara" }
                        p { class: "is-continue-chapter", "Allama Iqbal · Ch.14" }
                        div { class: "is-progress",
                            div { class: "is-progress-fill", style: "width: 68%" }
                        }
                        div { class: "is-progress-label",
                            span { "68%" }
                            span { "14 min left" }
                        }
                    }
                }
                hr { class: "is-divider" }
                div { class: "is-continue",
                    div { class: "is-continue-cover", style: "background: #f4dcc7",
                        Cover { urdu: "آگ", mono: "ق" }
                    }
                    div { class: "is-continue-info",
                        p { class: "is-continue-title", "Aag Ka Darya" }
                        p { class: "is-continue-chapter", "Part II · Champa" }
                        div { class: "is-progress",
                            div { class: "is-progress-fill", style: "width: 23%" }
                        }
                        div { class: "is-progress-label",
                            span { "23%" }
                            span { "4h 12m left" }
                        }
                    }
                }
            }

            div { class: "island is-rail-card",
                div { class: "is-rail-eyebrow", "Highlight from yesterday" }
                p { class: "is-quote",
                    "\u{201C}The night is long; even the moon is tired of waiting. \
                    Yet we sit, with the wine cooling, and pretend we are not.\u{201D}"
                }
                div { class: "is-quote-source", "Diwan-e-Ghalib · Ghazal 47" }
            }

            div { class: "island is-rail-card",
                div { class: "is-rail-eyebrow", "Reading Goal" }
                p { style: "margin: 8px 0 4px; font-family: var(--font-serif)",
                    span { style: "font-size: 28px; font-weight: 800; line-height: 1", "14" }
                    span { style: "color: var(--text-muted); font-size: 12px", " / 24 books in 2026" }
                }
                div { class: "is-progress", style: "height: 6px",
                    div { class: "is-progress-fill", style: "width: 58%" }
                }
                p { style: "font-size: 11.5px; color: var(--text-muted); margin: 8px 0 0; line-height: 1.4",
                    "You're 2 books ahead of pace. Next milestone: 18 by April end."
                }
            }
        }
    }
}
