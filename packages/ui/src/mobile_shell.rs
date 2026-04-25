use dioxus::prelude::*;
use crate::Route;

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
                svg {
                    class: "is-mob-tab-icon",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    view_box: "0 0 16 16",
                    path { d: "M2.5 7 L8 2.5 L13.5 7 V13 a1 1 0 0 1 -1 1 H3.5 a1 1 0 0 1 -1 -1 Z" }
                    path { d: "M6.5 14 V9 H9.5 V14" }
                }
                "Discover"
            }
            Link {
                to: Route::Shelf {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                svg {
                    class: "is-mob-tab-icon",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    view_box: "0 0 16 16",
                    path { d: "M4 2.5 H12 V14 L8 11 L4 14 Z" }
                }
                "Shelf"
            }
            Link {
                to: Route::Home {}, // Assuming authors route exists or use home for now
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                svg {
                    class: "is-mob-tab-icon",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    view_box: "0 0 16 16",
                    path { d: "M13 3 c-3 0 -7 1 -8.5 5 c-0.6 1.5 -0.6 3 0 4.5 L13 4.5 Z" }
                    path { d: "M8 8 H11" }
                    path { d: "M2.5 13.5 L6 10" }
                }
                "Authors"
            }
            Link {
                to: Route::Profile {},
                class: "is-mob-tab",
                active_class: "is-mob-tab--active",
                svg {
                    class: "is-mob-tab-icon",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.5",
                    view_box: "0 0 16 16",
                    circle { cx: "8", cy: "6", r: "3" }
                    path { d: "M3 13 c0 -3 2 -5 5 -5 s 5 2 5 5" }
                }
                "You"
            }
        }
    }
}
