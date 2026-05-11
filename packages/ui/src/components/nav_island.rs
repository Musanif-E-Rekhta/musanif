use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdSettings, LdUser},
    Icon,
};

use crate::state::CURRENT_USER;
use crate::Route;

/// Slack-style sidebar: brand block, grouped nav sections (Browse / Library),
/// then a flex spacer that pins Settings + the user pill to the bottom.
#[component]
pub fn NavIsland() -> Element {
    rsx! {
        aside { class: "island is-nav",
            div { class: "is-nav-brand",
                span { class: "is-nav-brand-mark", "مصنف" }
                span { class: "is-nav-brand-name", "Musanif" }
            }

            div { class: "is-nav-section", "Browse" }
            Link {
                to: Route::Home {},
                class: "is-nav-item",
                active_class: "is-nav-item--active",
                Icon { icon: LdCompass, width: 16, height: 16, class: "is-nav-item-icon" }
                "Discover"
            }
            Link {
                to: Route::Authors {},
                class: "is-nav-item",
                active_class: "is-nav-item--active",
                Icon { icon: LdPenLine, width: 16, height: 16, class: "is-nav-item-icon" }
                "Authors"
            }

            div { class: "is-nav-section", "Library" }
            Link {
                to: Route::Shelf {},
                class: "is-nav-item",
                active_class: "is-nav-item--active",
                Icon { icon: LdLibrary, width: 16, height: 16, class: "is-nav-item-icon" }
                "My Shelf"
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
                    if let Some(user) = CURRENT_USER.read().as_ref() {
                        {user.username.chars().next().unwrap_or('?').to_ascii_uppercase().to_string()}
                    } else {
                        Icon { icon: LdUser, width: 16, height: 16 }
                    }
                }
                div { style: "flex: 1",
                    if let Some(user) = CURRENT_USER.read().as_ref() {
                        div { class: "is-nav-user-name", "{user.username}" }
                        div { class: "is-nav-user-meta", "View profile" }
                    } else {
                        div { class: "is-nav-user-name", "Guest" }
                        div { class: "is-nav-user-meta", "Not signed in" }
                    }
                }
            }
        }
    }
}
