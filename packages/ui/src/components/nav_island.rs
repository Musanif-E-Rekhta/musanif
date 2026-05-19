use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdSettings},
    Icon,
};

use crate::state::CURRENT_USER;
use crate::Route;

/// Slack-style sidebar: brand anchor, grouped nav sections (Browse / Library),
/// flex spacer, then Settings and a quiet typographic identity line at the foot.
#[component]
pub fn NavIsland() -> Element {
    rsx! {
        aside { class: "island is-nav",
            Link {
                to: Route::Home {},
                class: "is-nav-brand",
                "aria-label": "Musanif home",
                span { class: "is-nav-brand-mark", dir: "rtl", lang: "ur", "مصنف" }
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
                if let Some(user) = CURRENT_USER.read().as_ref() {
                    div { class: "is-nav-user-name", "{user.username}" }
                } else {
                    div { class: "is-nav-user-name", "Guest" }
                    div { class: "is-nav-user-meta", "Sign in" }
                }
            }
        }
    }
}
