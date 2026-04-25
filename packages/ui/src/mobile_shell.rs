use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCompass, LdLibrary, LdPenLine, LdUser},
    Icon,
};

#[component]
pub fn MobileShell() -> Element {
    rsx! {
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
