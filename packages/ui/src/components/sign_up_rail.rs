use dioxus::prelude::*;

use super::HighlightCard;
use crate::Route;

/// Right-rail variant for signed-out Discover. The catalog is browsable
/// without an account, but the rail stops carrying personal context
/// (Continue Reading, Reading Goal) and instead offers a single
/// hospitable invitation to sign in. A curated highlight quote still
/// renders below: it does cultural work for any visitor and reinforces
/// the literary tone before the user has authenticated.
#[component]
pub fn SignUpRail() -> Element {
    rsx! {
        div { class: "is-rail-col",
            section { class: "island is-rail-card is-signup-card",
                p { class: "is-rail-eyebrow", "Save your reading" }
                p { class: "is-signup-headline",
                    "Pick up where you left off, keep what you love, and set a reading goal for the year."
                }
                div { class: "is-signup-actions",
                    Link {
                        to: Route::Login {},
                        class: "is-btn is-btn--primary is-btn--block",
                        "Sign in"
                    }
                    Link {
                        to: Route::Signup {},
                        class: "is-signup-secondary",
                        "Create an account"
                    }
                }
            }

            HighlightCard {
                eyebrow: Some("From the corpus".to_string()),
                quote: "Hazaaron khwahishein aisi ke har khwahish pe dam nikle.".to_string(),
                source: Some("Ghalib".to_string()),
            }
        }
    }
}
