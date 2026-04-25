use dioxus::prelude::*;

use crate::Route;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// Set to `true` from outside the router (e.g. a custom desktop titlebar)
/// to trigger a home navigation from inside the router context.
pub static NAVIGATE_HOME: GlobalSignal<bool> = Signal::global(|| false);

/// Generic slot-based navbar used by legacy platform wrappers.
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { id: "navbar", {children} }
    }
}

/// Router layout: vertical island sidebar + scrollable main content.
#[component]
pub fn AppNavbar() -> Element {
    let nav = use_navigator();
    let _current_route = use_route::<Route>();

    use_effect(move || {
        if *NAVIGATE_HOME.read() {
            *NAVIGATE_HOME.write() = false;
            nav.push(Route::Home {});
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { class: "is-shell",
            aside { class: "island is-nav",
                div { class: "is-nav-brand",
                    span { class: "is-nav-brand-mark", "مصنف" }
                    span { class: "is-nav-brand-name", "Musanif" }
                }

                Link {
                    to: Route::Home {},
                    class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
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
                    class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 16 16",
                        path { d: "M4 2.5 H12 V14 L8 11 L4 14 Z" }
                    }
                    "My Shelf"
                    span { class: "is-nav-item-count", "12" }
                }
                Link {
                    to: Route::Authors {},
                    class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
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
                    to: Route::Home {},
                    class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 16 16",
                        circle { cx: "7", cy: "7", r: "4.5" }
                        path { d: "M10.5 10.5 L13.5 13.5" }
                    }
                    "Search"
                }

                div { class: "is-nav-section", "Reading" }
                button { class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 16 16",
                        path { d: "M3 2.5 H7 a2 2 0 0 1 2 2 V14 H4 a1 1 0 0 1 -1 -1 Z" }
                        path { d: "M13 2.5 H9 V14 H12 a1 1 0 0 0 1 -1 Z" }
                    }
                    "Continue · Ch.47"
                }
                button { class: "is-nav-item",
                    svg {
                        class: "is-nav-item-icon",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 16 16",
                        path { d: "M8 2 L9 6 L13 7 L9 8 L8 12 L7 8 L3 7 L7 6 Z" }
                    }
                    "Highlights"
                }

                div { class: "is-nav-spacer" }

                Link {
                    to: Route::Profile {},
                    class: "is-nav-item",
                    div { class: "is-nav-user-avatar", "U" }
                    div { style: "flex: 1",
                        div { class: "is-nav-user-name", "Usaira Imisani" }
                        div { class: "is-nav-user-meta", "Reader · 142 books" }
                    }
                    span {
                        onclick: move |e| {
                            e.stop_propagation();
                            nav.push(Route::Settings {});
                        },
                        style: "color: var(--text-muted); padding: 4px; cursor: pointer; display: flex",
                        svg {
                            width: "18",
                            height: "18",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                    }
                }
            }

            main { class: "is-main",
                Outlet::<Route> {}
            }
            crate::components::ThemeSwitcher {}
        }
    }
}
