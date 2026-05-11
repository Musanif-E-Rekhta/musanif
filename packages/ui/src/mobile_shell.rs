use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdUser},
    Icon,
};

const SHARED_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn MobileShell() -> Element {
    use_effect(crate::theme::load_persisted);

    rsx! {
        // Load the shared primitives (.island, .is-progress, .is-quote,
        // .is-reader-*, .is-chip, .is-btn, …) before the mobile-specific
        // stylesheet so platform overrides win.
        document::Link { rel: "stylesheet", href: SHARED_CSS }

        div { class: "is-mob",
            div { class: "is-mob-statusbar",
                span { "9:41" }
                div { class: "is-mob-statusbar-icons",
                    span { "●●●" }
                    span { "📶" }
                    span { "100%" }
                }
            }

            div { class: "is-mob-body",
                Outlet::<Route> {}
            }

            MobileTabs {}

            crate::components::ThemeSwitcher {}
        }
    }
}

#[component]
fn MobileTabs() -> Element {
    rsx! {
        nav { class: "is-mob-tabs",
            Link {
                to: Route::Home {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                Icon { icon: LdCompass, width: 20, height: 20, class: "is-mob-tab-icon" }
                "Discover"
            }
            Link {
                to: Route::Shelf {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                Icon { icon: LdLibrary, width: 20, height: 20, class: "is-mob-tab-icon" }
                "Shelf"
            }
            Link {
                to: Route::Authors {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                Icon { icon: LdPenLine, width: 20, height: 20, class: "is-mob-tab-icon" }
                "Authors"
            }
            Link {
                to: Route::Profile {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                Icon { icon: LdUser, width: 20, height: 20, class: "is-mob-tab-icon" }
                "You"
            }
        }
    }
}
