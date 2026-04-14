use dioxus::prelude::*;

use ui::Navbar;
use views::{Blog, Home};

mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(DesktopNavbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    let cfg = dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_decorations(false));

    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn TitleBar() -> Element {
    let window = dioxus::desktop::window();

    rsx! {
        div {
            id: "titlebar",
            onmousedown: move |_| {
                dioxus::desktop::window().drag();
            },
            div { class: "titlebar-left",
                img {
                    class: "titlebar-icon",
                    src: asset!("/assets/musanif.png")
                }
                "Musanif"
            }
            div { class: "titlebar-controls",
                button {
                    onclick: move |_| {
                        info!("M");dioxus::desktop::window().set_minimized(true)},
                    "‒"
                }
                button {
                    onclick: move |_| {
                        let w = dioxus::desktop::window();
                        w.set_maximized(!w.is_maximized());
                    },
                    "◻"
                }
                button {
                    class: "close",
                    onclick: move |_| dioxus::desktop::window().close(),
                    "✕"
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        TitleBar {}

        div {
            class: "content",
            Router::<Route> {}
        }
    }
}

/// A desktop-specific Router around the shared `Navbar` component
/// which allows us to use the desktop-specific `Route` enum.
#[component]
fn DesktopNavbar() -> Element {
    rsx! {
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}
