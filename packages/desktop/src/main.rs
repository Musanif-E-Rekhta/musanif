use dioxus::desktop::tao::window::ResizeDirection;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    let cfg = dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_decorations(false));
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn TitleBar() -> Element {
    rsx! {
        div {
            id: "titlebar",
            onmousedown: move |_| { dioxus::desktop::window().drag(); },
            div {
                class: "titlebar-controls",
                // Stop propagation so button clicks don't trigger the titlebar drag.
                onmousedown: move |e| e.stop_propagation(),
                button {
                    class: "close",
                    onclick: move |_| dioxus::desktop::window().close(),
                    "✕"
                }
                button {
                    onclick: move |_| {
                        let w = dioxus::desktop::window();
                        w.set_maximized(!w.is_maximized());
                    },
                    "◻"
                }
                button {
                    onclick: move |_| { dioxus::desktop::window().set_minimized(true) },
                    "‒"
                }
            }
            div {
                class: "titlebar-brand",
                onmousedown: move |e| e.stop_propagation(),
                onclick: move |_| { *ui::NAVIGATE_HOME.write() = true; },
                "مصنف"
                img { class: "titlebar-icon", src: asset!("/assets/musanif.png") }
            }
        }
    }
}

#[component]
fn ResizeHandles() -> Element {
    rsx! {
        div { class: "resize-handle resize-nw",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::NorthWest); }
        }
        div { class: "resize-handle resize-n",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::North); }
        }
        div { class: "resize-handle resize-ne",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::NorthEast); }
        }
        div { class: "resize-handle resize-e",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::East); }
        }
        div { class: "resize-handle resize-se",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::SouthEast); }
        }
        div { class: "resize-handle resize-s",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::South); }
        }
        div { class: "resize-handle resize-sw",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::SouthWest); }
        }
        div { class: "resize-handle resize-w",
            onmousedown: move |e| { e.stop_propagation(); let _ = dioxus::desktop::window().drag_resize_window(ResizeDirection::West); }
        }
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ResizeHandles {}
        TitleBar {}
        div { class: "content", Router::<ui::Route> {} }
    }
}
