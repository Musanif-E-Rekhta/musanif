use dioxus::prelude::*;

use crate::components::{Field, FontSample, SegOption, SegRow, Toggle};
use crate::prefs;

#[component]
pub fn ReadingPrefs() -> Element {
    let mut font_size = prefs::use_persisted_u32("reading.font_size", 17);
    let mut line_height = prefs::use_persisted_u32("reading.line_height", 175); // × 0.01 = 1.75
    let typeface = prefs::use_persisted_string("reading.typeface", "serif");
    let mut dictionary = prefs::use_persisted_bool("reading.dictionary", true);

    let font_family = match typeface.read().as_str() {
        "sans" => "var(--font-sans)",
        "urdu" => "var(--font-urdu)",
        _ => "var(--font-serif)",
    };

    let preview_text = "There is a way of waiting that does not look like waiting. The cup sits, and the night sits, and one is not aware that anything is being measured.";

    rsx! {
        div {
            div {
                class: "settings-preview",
                style: "font-size: {font_size()}px; line-height: {line_height() as f32 / 100.0}; font-family: {font_family}",
                "{preview_text}"
            }

            Field {
                label: "Typeface".to_string(),
                hint: Some("Affects all chapter body text.".to_string()),
                SegRow {
                    SegOption { value: "serif".to_string(), active: typeface, "Serif" }
                    SegOption { value: "sans".to_string(), active: typeface, "Sans" }
                    SegOption { value: "urdu".to_string(), active: typeface, "Nastaliq" }
                }
            }

            Field {
                label: "Font samples".to_string(),
                hint: Some("Pick the headline face that suits your eye.".to_string()),
                div { class: "settings-theme-grid",
                    FontSample {
                        value: "serif".to_string(),
                        label: "Serif".to_string(),
                        sample: "Aa".to_string(),
                        font_family: "var(--font-serif)".to_string(),
                        active: typeface,
                    }
                    FontSample {
                        value: "sans".to_string(),
                        label: "Sans".to_string(),
                        sample: "Aa".to_string(),
                        font_family: "var(--font-sans)".to_string(),
                        active: typeface,
                    }
                    FontSample {
                        value: "urdu".to_string(),
                        label: "Nastaliq".to_string(),
                        sample: "غ".to_string(),
                        font_family: "var(--font-urdu)".to_string(),
                        active: typeface,
                    }
                }
            }

            label { class: "settings-field-label", "Font size · {font_size()}px" }
            input {
                r#type: "range",
                class: "settings-range",
                min: "14",
                max: "24",
                value: "{font_size()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { font_size.set(v); }
                }
            }

            label { class: "settings-field-label", "Line height · {line_height() as f32 / 100.0:.2}" }
            input {
                r#type: "range",
                class: "settings-range",
                min: "140",
                max: "200",
                step: "5",
                value: "{line_height()}",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { line_height.set(v); }
                }
            }

            Field {
                label: "Dictionary".to_string(),
                hint: Some("Show word meanings on double-tap.".to_string()),
                Toggle {
                    on: *dictionary.read(),
                    onchange: move |v: bool| dictionary.set(v),
                }
            }
        }
    }
}
