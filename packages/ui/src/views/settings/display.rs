use dioxus::prelude::*;

use super::Toggle;
use crate::{prefs, theme};

#[component]
pub fn DisplayPrefs() -> Element {
    let themes = [
        ("parchment", "Parchment", "#f4ede1", "#b8412f", "Default warm light"),
        ("midnight", "Midnight", "#0f1216", "#ec7666", "Cool dark"),
        ("sepia-dark", "Sepia Dark", "#1c1610", "#ec8775", "Warm dark"),
        ("ink", "Ink", "#000000", "#ed7561", "Pure black, OLED"),
    ];
    let mut current_theme = prefs::use_persisted_string("theme", "parchment");
    let mut auto_sunset = prefs::use_persisted_bool("display.auto_sunset", false);
    let mut show_urdu_glyphs = prefs::use_persisted_bool("display.show_urdu_glyphs", true);

    rsx! {
        div {
            h3 { class: "settings-h3", "Display & theme" }

            label { class: "settings-field-label", "Theme" }
            div { class: "settings-theme-grid", style: "margin-bottom: 22px",
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
                            span { style: "color: var(--primary)", "✓" }
                        }
                    }
                }
            }

            label { class: "settings-field-label", "Auto-switch at sunset" }
            p { class: "settings-field-hint", "Use Midnight theme between 7pm and 7am" }
            Toggle {
                on: *auto_sunset.read(),
                onchange: move |v: bool| auto_sunset.set(v),
            }

            div { style: "margin-top: 22px" }
            label { class: "settings-field-label", "Show Urdu glyphs in UI" }
            p { class: "settings-field-hint", "Display book titles in original script alongside Roman" }
            Toggle {
                on: *show_urdu_glyphs.read(),
                onchange: move |v: bool| show_urdu_glyphs.set(v),
            }
        }
    }
}
