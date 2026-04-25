use dioxus::prelude::*;

use crate::Route;

const PROFILE_CSS: Asset = asset!("/assets/styling/profile.css");

#[component]
pub fn Profile() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: PROFILE_CSS }

        div { class: "profile-page",

            // ── Header ──────────────────────────────────────────────
            div { class: "profile-header",
                div { class: "profile-avatar", "R" }
                div { class: "profile-info",
                    h1 { class: "profile-name", "Reader" }
                    p { class: "profile-email", "reader@example.com" }
                }
            }

            // ── Stats ────────────────────────────────────────────────
            div { class: "profile-section",
                h2 { class: "profile-section-title", "Reading Stats" }
                div { class: "stats-grid",
                    div { class: "stat-card",
                        span { class: "stat-value", "0" }
                        span { class: "stat-label", "Books Read" }
                    }
                    div { class: "stat-card",
                        span { class: "stat-value", "0" }
                        span { class: "stat-label", "Reading Now" }
                    }
                    div { class: "stat-card",
                        span { class: "stat-value", "0" }
                        span { class: "stat-label", "Want to Read" }
                    }
                }
            }

            // ── Account ──────────────────────────────────────────────
            div { class: "profile-section",
                h2 { class: "profile-section-title", "Account" }
                div { class: "profile-actions",
                    button { class: "profile-action-btn", "Edit Profile" }
                    Link {
                        to: Route::Login {},
                        class: "profile-action-btn profile-action-btn--danger",
                        "Sign Out"
                    }
                }
            }
        }
    }
}
