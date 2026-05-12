use dioxus::prelude::*;

use super::Toggle;
use crate::{prefs, theme};

#[component]
pub fn DisplayPrefs() -> Element {
    let themes = [
        (
            "parchment",
            "Parchment",
            "#f4ede1",
            "#b8412f",
            "Warm light for daytime.",
        ),
        (
            "midnight",
            "Midnight",
            "#0f1216",
            "#ec7666",
            "Lamplit evening.",
        ),
        (
            "sepia-dark",
            "Sepia Dark",
            "#1c1610",
            "#ec8775",
            "Worn manuscript.",
        ),
        (
            "ink",
            "Ink",
            "#000000",
            "#ed7561",
            "True black for OLED nights.",
        ),
    ];
    let mut current_theme = prefs::use_persisted_string("theme", "parchment");
    let mut auto_sunset = prefs::use_persisted_bool("display.auto_sunset", false);
    let mut show_urdu_glyphs = prefs::use_persisted_bool("display.show_urdu_glyphs", true);

    rsx! {
        div { class: "settings-pane",
            div { class: "settings-theme-block",
                label { class: "settings-field-label", "Theme" }
                div { class: "settings-theme-grid",
                    for (id, label, bg, primary, desc) in themes {
                        button {
                            key: "{id}",
                            class: if *current_theme.read() == id { "settings-theme-btn settings-theme-btn--active" } else { "settings-theme-btn settings-theme-btn--inactive" },
                            onclick: {
                                let id = id.to_string();
                                move |_| {
                                    current_theme.set(id.clone());
                                    theme::apply_and_persist(&id);
                                }
                            },
                            div { class: "settings-theme-swatch", style: "background: {bg}",
                                span { class: "settings-theme-swatch-dot", style: "background: {primary}" }
                            }
                            div { style: "flex: 1",
                                p { class: "settings-theme-name", "{label}" }
                                p { class: "settings-theme-desc", "{desc}" }
                            }
                            if *current_theme.read() == id {
                                span { class: "settings-theme-check", "✓" }
                            }
                        }
                    }
                }
            }

            div { class: "settings-toggle-row",
                div {
                    label { class: "settings-field-label", "Auto-switch at sunset" }
                    p { class: "settings-field-hint", "Use Midnight between 7pm and 7am." }
                }
                Toggle {
                    on: *auto_sunset.read(),
                    onchange: move |v: bool| auto_sunset.set(v),
                }
            }

            div { class: "settings-toggle-row",
                div {
                    label { class: "settings-field-label", "Show Urdu glyphs in chrome" }
                    p { class: "settings-field-hint",
                        "Book titles render in original script alongside Roman."
                    }
                }
                Toggle {
                    on: *show_urdu_glyphs.read(),
                    onchange: move |v: bool| show_urdu_glyphs.set(v),
                }
            }
        }
    }
}
