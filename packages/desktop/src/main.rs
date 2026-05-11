//! Desktop (tao/wry) entry point for the Musanif reader.
//!
//! Builds a frameless window and adds a custom title bar (drag region
//! + breadcrumb + window controls) on top of [`ui::Route`]. The
//! decoration-less `WindowBuilder` and the `WindowTab` /
//! `WindowTabStrip` chrome live here because they're desktop-specific;
//! everything else comes from [`ui`].

use dioxus::desktop::tao::window::ResizeDirection;
use dioxus::prelude::*;
use ui::components::{WindowTab, WindowTabStrip};

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

/// Tab strip below the chrome — populated from the user's `reading`
/// bookmarks. The plan calls for a "recent reading proxy" while we don't
/// have a real open-books model. Empty space drags the window; tabs and
/// the close button stop propagation to keep their clicks intact.
#[component]
fn ChapterTabStrip() -> Element {
    let bookmarks = use_resource(move || async move {
        ui::api::fetch_my_bookmarks(Some("reading".to_string())).await
    });

    let active_slug = ui::CURRENT_BOOK_SLUG.read().clone();

    rsx! {
        div {
            class: "dt-tabstrip",
            onmousedown: move |_| { dioxus::desktop::window().drag(); },

            match &*bookmarks.read() {
                Some(Some(bms)) if !bms.is_empty() => rsx! {
                    WindowTabStrip {
                        for bm in bms.iter().filter(|b| b.book.is_some()).take(8) {
                            {
                                let book = bm.book.as_ref().expect("filtered above");
                                let slug = book.slug.clone();
                                let title = book.title.clone();
                                let glyph = book.title.chars().next().map(|c| c.to_string());
                                let active = active_slug.as_deref() == Some(slug.as_str());
                                rsx! {
                                    WindowTab {
                                        key: "{slug}",
                                        title: title,
                                        glyph: glyph,
                                        active: active,
                                        onclick: {
                                            let slug = slug.clone();
                                            move |_| { *ui::NAVIGATE_TO_BOOK_SLUG.write() = Some(slug.clone()); }
                                        },
                                    }
                                }
                            }
                        }
                    }
                },
                _ => rsx! {
                    span {
                        class: "dt-tabstrip-empty",
                        onmousedown: move |e| e.stop_propagation(),
                        "No books in progress · open one to pin it here"
                    }
                },
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
        ui::theme::apply_and_persist(ui::CURRENT_THEME().as_str());
    });

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        ResizeHandles {}
        div { class: "dt-window dt-window--with-tabs",
            WindowChrome {}
            ChapterTabStrip {}
            div { class: "dt-window-body",
                Router::<ui::Route> {}
            }
        }
    }
}
