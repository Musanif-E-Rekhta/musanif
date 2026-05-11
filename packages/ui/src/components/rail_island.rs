use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdSearch, LdSettings},
    Icon,
};

use crate::Route;

/// Collapsed icon-only sidebar used by the focus/reader shell.
/// Mirrors `AppNavbar`'s links but rendered as 38×38 icon tiles instead of
/// labelled rows; settings is pinned to the bottom by `is-rail-spacer`.
#[component]
pub fn RailIsland() -> Element {
    let current_route = use_route::<Route>();

    let is_active = |route: &Route| -> bool {
        std::mem::discriminant(&current_route) == std::mem::discriminant(route)
    };

    rsx! {
        aside { class: "island is-rail",
            div { class: "is-rail-brand", "م" }
            Link {
                to: Route::Home {},
                class: if is_active(&Route::Home {}) { "is-rail-item is-rail-item--active" } else { "is-rail-item" },
                Icon { icon: LdCompass, width: 18, height: 18 }
            }
            Link {
                to: Route::Shelf {},
                class: if is_active(&Route::Shelf {}) { "is-rail-item is-rail-item--active" } else { "is-rail-item" },
                Icon { icon: LdLibrary, width: 18, height: 18 }
            }
            Link {
                to: Route::Authors {},
                class: if is_active(&Route::Authors {}) { "is-rail-item is-rail-item--active" } else { "is-rail-item" },
                Icon { icon: LdPenLine, width: 18, height: 18 }
            }
            button { class: "is-rail-item",
                Icon { icon: LdSearch, width: 18, height: 18 }
            }
            div { class: "is-rail-spacer" }
            Link {
                to: Route::Settings {},
                class: if is_active(&Route::Settings {}) { "is-rail-item is-rail-item--active" } else { "is-rail-item" },
                Icon { icon: LdSettings, width: 18, height: 18 }
            }
        }
    }
}
