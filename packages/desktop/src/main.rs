use dioxus::desktop::tao::window::ResizeDirection;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    let cfg = dioxus::desktop::Config::new()
        .with_window(dioxus::desktop::WindowBuilder::new().with_decorations(false));
    dioxus::LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

/// Title bar: full-width drag region with a breadcrumb on the left and window controls on the right.
#[component]
fn WindowChrome() -> Element {
    let page_title = ui::CURRENT_PAGE_TITLE.read();

    rsx! {
        div {
            class: "dt-window-chrome",
            onmousedown: move |_| { dioxus::desktop::window().drag(); },

            // Breadcrumb — clicking root navigates home; stop_propagation only on the click, not mousedown
            div { class: "dt-breadcrumb",
                span {
                    class: "dt-breadcrumb-root",
                    onclick: move |_| { *ui::NAVIGATE_HOME.write() = true; },
                    "Musanif"
                }
                if !page_title.is_empty() {
                    span { class: "dt-breadcrumb-sep", "›" }
                    span { class: "dt-breadcrumb-current", "{*page_title}" }
                }
            }

            // Window controls — swallow mousedown to prevent drag
            div {
                class: "dt-controls",
                onmousedown: move |e| e.stop_propagation(),
                button {
                    class: "dt-control-btn dt-control-btn--min",
                    onclick: move |_| dioxus::desktop::window().set_minimized(true),
                    "—"
                }
                button {
                    class: "dt-control-btn dt-control-btn--max",
                    onclick: move |_| {
                        let w = dioxus::desktop::window();
                        w.set_maximized(!w.is_maximized());
                    },
                    "❐"
                }
                button {
                    class: "dt-control-btn dt-control-btn--close",
                    onclick: move |_| dioxus::desktop::window().close(),
                    "✕"
                }
            }
        }
    }
}

/// Invisible edge/corner hit-targets for resizing the frameless window.
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
    use_effect(move || {
        let theme_str = ui::CURRENT_THEME().as_str();
        let _ = document::eval(&format!(
            "document.documentElement.setAttribute('data-theme', '{theme_str}'); \
             try {{ localStorage.setItem('musanif-theme', '{theme_str}'); }} catch(e) {{}}"
        ));
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ResizeHandles {}
        div { class: "dt-window",
            WindowChrome {}
            div { class: "dt-window-body",
                Router::<ui::Route> {}
            }
        }
    }
}
