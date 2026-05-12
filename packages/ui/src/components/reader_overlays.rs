//! Reader overlays that aren't anchored to the topbar chrome: the
//! top-edge progress thread, the selection-anchored highlight toolbar,
//! and the keyboard shortcut overlay.
//!
//! Each piece is rendered unconditionally so its transitions run in
//! both directions; visibility is driven by class flips, not by
//! mounting and unmounting. The parent in
//! [`crate::views::chapter_reader`] owns the signals and feeds them in.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Selection metadata sent from the chapter_reader JS bridge whenever
/// the user finishes selecting non-empty text inside `.is-reader-body`.
/// Coordinates are in viewport (CSS) pixels; offsets are character
/// indices relative to the parent paragraph's `textContent`.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SelectionInfo {
    pub x: f64,
    pub y: f64,
    pub paragraph: i32,
    pub offset_start: i32,
    pub offset_end: i32,
    pub text: String,
}

/// Top-edge scroll progress thread. Always visible; survives the
/// topbar's auto-hide so the reader always knows how far they are.
#[component]
pub fn ReaderProgressBar(pct: Signal<f64>) -> Element {
    let value = pct.read().clamp(0.0, 100.0);
    rsx! {
        div { class: "is-reader-progress",
            role: "progressbar",
            "aria-valuemin": 0,
            "aria-valuemax": 100,
            "aria-valuenow": format!("{value:.0}"),
            "aria-label": "Reading progress",
            div { class: "is-reader-progress-fill", style: "width: {value}%" }
        }
    }
}

/// Floating toolbar anchored above an active selection. Picks a color
/// and persists immediately; offers a note input collapsed by default.
#[component]
pub fn HighlightToolbar(
    selection: Signal<Option<SelectionInfo>>,
    on_highlight: EventHandler<(SelectionInfo, &'static str, Option<String>)>,
) -> Element {
    let snap = selection.read().clone();
    let mut note_open = use_signal(|| false);
    let mut note_text = use_signal(String::new);
    let mut pending_color = use_signal(|| None::<&'static str>);

    // Reset internal state when selection clears, so a new selection
    // starts fresh.
    use_effect(move || {
        if selection.read().is_none() {
            note_open.set(false);
            note_text.set(String::new());
            pending_color.set(None);
        }
    });

    let info = match snap {
        Some(s) => s,
        None => return rsx! { Fragment {} },
    };

    // Anchor the toolbar 12px above the selection's top, horizontally
    // centered on its midpoint; the stylesheet handles the translate
    // so the inline style is just the absolute origin.
    let top_px = (info.y - 48.0).max(8.0);
    let left_px = info.x.max(8.0);

    let colors: &[(&str, &str, &str)] = &[
        ("warm", "is-highlight--warm", "Warm yellow"),
        ("terracotta", "is-highlight--terracotta", "Terracotta"),
        ("sage", "is-highlight--sage", "Sage"),
        ("lavender", "is-highlight--lavender", "Lavender"),
    ];

    rsx! {
        div { class: "is-highlight-toolbar",
            style: "top: {top_px}px; left: {left_px}px",
            role: "toolbar",
            "aria-label": "Highlight",

            if *note_open.read() {
                NoteForm {
                    text: note_text,
                    on_save: move |_| {
                        let color = pending_color.read().unwrap_or("warm");
                        let txt = note_text.read().clone();
                        let note = if txt.trim().is_empty() { None } else { Some(txt) };
                        on_highlight.call((info.clone(), color, note));
                        selection.set(None);
                    },
                    on_cancel: move |_| {
                        note_open.set(false);
                        note_text.set(String::new());
                    },
                }
            } else {
                div { class: "is-highlight-toolbar-row",
                    for (key, color_class, aria) in colors.iter() {
                        button {
                            key: "{key}",
                            class: "is-highlight-swatch {color_class}",
                            "aria-label": "{aria}",
                            onclick: {
                                let info = info.clone();
                                let key = *key;
                                move |_| {
                                    on_highlight.call((info.clone(), key, None));
                                    selection.set(None);
                                }
                            },
                        }
                    }

                    span { class: "is-highlight-toolbar-divider" }

                    button {
                        class: "is-highlight-toolbar-action",
                        "aria-label": "Add a note",
                        onclick: move |_| {
                            if pending_color.read().is_none() {
                                pending_color.set(Some("warm"));
                            }
                            note_open.set(true);
                        },
                        "Note"
                    }

                    button {
                        class: "is-highlight-toolbar-action",
                        "aria-label": "Dismiss",
                        onclick: move |_| selection.set(None),
                        "✕"
                    }
                }
            }
        }
    }
}

#[component]
fn NoteForm(
    text: Signal<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
) -> Element {
    rsx! {
        div { class: "is-highlight-note",
            textarea {
                class: "is-highlight-note-input",
                rows: 3,
                placeholder: "Note to self, or a question for the margin.",
                value: "{text}",
                autofocus: true,
                oninput: move |e| text.set(e.value()),
            }
            div { class: "is-highlight-note-actions",
                button {
                    class: "is-btn",
                    onclick: move |_| on_cancel.call(()),
                    "Cancel"
                }
                button {
                    class: "is-btn is-btn--primary",
                    onclick: move |_| on_save.call(()),
                    "Save"
                }
            }
        }
    }
}

/// Modal-ish overlay listing every reader shortcut. Esc closes,
/// click-on-scrim closes; nothing else.
#[component]
pub fn KeyboardOverlay(open: Signal<bool>) -> Element {
    if !*open.read() {
        return rsx! { Fragment {} };
    }

    let rows: &[(&str, &str)] = &[
        ("← / →", "Previous / next chapter"),
        ("J / K", "Scroll a paragraph forward / back"),
        ("T", "Open the typography sheet"),
        ("B", "Bookmark this chapter"),
        ("O", "Open the chapter list"),
        ("?", "Show this overlay"),
        ("Esc", "Close any open overlay"),
    ];

    rsx! {
        div { class: "is-kbd-scrim",
            onclick: move |_| open.set(false),

            div { class: "is-kbd-overlay",
                role: "dialog",
                "aria-label": "Shortcuts",
                onclick: move |e| e.stop_propagation(),

                p { class: "is-kbd-eyebrow", "SHORTCUTS" }
                h2 { class: "is-kbd-title", "Read with your hands on the keyboard." }

                dl { class: "is-kbd-list",
                    for (k, label) in rows.iter() {
                        div { key: "{k}", class: "is-kbd-row",
                            dt { class: "is-kbd-keys", "{k}" }
                            dd { class: "is-kbd-label", "{label}" }
                        }
                    }
                }

                p { class: "is-kbd-footer",
                    "Hold "
                    span { class: "is-kbd-inline", "?" }
                    " for as long as you need."
                }
            }
        }
    }
}
