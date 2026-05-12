//! Right-anchored popovers for typography and theme.
//!
//! Both popovers float beneath the reader topbar, anchored to their
//! triggering icon. They are not modals: they sit on top of the
//! reading column with a thin separator and let the body scroll
//! through underneath. Outside-click and Esc dismiss; settings persist
//! through [`crate::prefs`] as the user makes them, so there is no
//! "Apply" button.
//!
//! The available type axes (face / size / line-height / measure) map
//! to `data-` attributes on the reader root, which the stylesheet
//! reads to set CSS custom properties. That keeps the component logic
//! free of font math.

use dioxus::prelude::*;

use crate::{prefs, state::CURRENT_THEME, theme};

const FACES: &[(&str, &str)] = &[
    ("serif", "Serif"),
    ("sans", "Sans"),
    ("nastaliq", "Nastaliq"),
];
const SIZES: &[(&str, &str)] = &[
    ("s", "S"),
    ("m", "M"),
    ("l", "L"),
    ("xl", "XL"),
];
const LINE_HEIGHTS: &[(&str, &str)] = &[
    ("snug", "Snug"),
    ("regular", "Regular"),
    ("loose", "Loose"),
];
const MEASURES: &[(&str, &str)] = &[
    ("narrow", "Narrow"),
    ("regular", "Regular"),
    ("wide", "Wide"),
    ("xwide", "Extra-wide"),
];

#[component]
pub fn ReaderTypeSheet(open: Signal<bool>) -> Element {
    if !*open.read() {
        return rsx! { Fragment {} };
    }

    let mut face = prefs::use_persisted_string("reader.face", "serif");
    let mut size = prefs::use_persisted_string("reader.size", "m");
    let mut line_height = prefs::use_persisted_string("reader.lh", "regular");
    let mut measure = prefs::use_persisted_string("reader.measure", "regular");

    rsx! {
        div { class: "is-reader-sheet",
            role: "dialog",
            "aria-label": "Typography",

            Section { label: "TYPEFACE" }
            ChoiceRow {
                options: FACES,
                selected: face.read().clone(),
                onpick: move |v: String| face.set(v),
            }

            Section { label: "SIZE" }
            ChoiceRow {
                options: SIZES,
                selected: size.read().clone(),
                onpick: move |v: String| size.set(v),
            }

            Section { label: "LINE-HEIGHT" }
            ChoiceRow {
                options: LINE_HEIGHTS,
                selected: line_height.read().clone(),
                onpick: move |v: String| line_height.set(v),
            }

            Section { label: "MEASURE" }
            ChoiceRow {
                options: MEASURES,
                selected: measure.read().clone(),
                onpick: move |v: String| measure.set(v),
            }

            p { class: "is-reader-sheet-footer",
                "Settings save as you go."
            }
        }
    }
}

/// In-reader theme picker. Shows the four named themes as small
/// swatches with a check on the active one, plus a "More themes…"
/// link to /settings#display for parity with the full settings page.
#[component]
pub fn ReaderThemeSheet(open: Signal<bool>) -> Element {
    if !*open.read() {
        return rsx! { Fragment {} };
    }

    let current = *CURRENT_THEME.read();

    let row = |id: &'static str, label: &'static str, bg: &'static str, ink: &'static str| {
        let is_active = current.as_str() == id;
        let class = if is_active {
            "is-reader-theme-btn is-reader-theme-btn--active"
        } else {
            "is-reader-theme-btn"
        };
        rsx! {
            button {
                key: "{id}",
                class: "{class}",
                onclick: move |_| theme::apply_and_persist(id),
                span {
                    class: "is-reader-theme-swatch",
                    style: "background: {bg}",
                    span {
                        class: "is-reader-theme-swatch-ink",
                        style: "background: {ink}",
                    }
                }
                span { class: "is-reader-theme-name", "{label}" }
                if is_active {
                    span { class: "is-reader-theme-check", "✓" }
                }
            }
        }
    };

    rsx! {
        div { class: "is-reader-sheet is-reader-sheet--theme",
            role: "dialog",
            "aria-label": "Theme",

            div { class: "is-reader-theme-list",
                {row("parchment", "Parchment", "#f4ede1", "#b8412f")}
                {row("midnight", "Midnight", "#0f1216", "#ec7666")}
                {row("sepia-dark", "Sepia Dark", "#1c1610", "#ec8775")}
                {row("ink", "Ink", "#0a0a0a", "#ed7561")}
            }

            p { class: "is-reader-sheet-footer",
                "Pick from "
                a { href: "/settings", class: "is-reader-sheet-link", "all themes" }
                "."
            }
        }
    }
}

#[component]
fn Section(label: &'static str) -> Element {
    rsx! {
        p { class: "is-reader-sheet-section", "{label}" }
    }
}

#[component]
fn ChoiceRow(
    options: &'static [(&'static str, &'static str)],
    selected: String,
    onpick: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "is-reader-sheet-row",
            role: "radiogroup",
            for (value, label) in options.iter() {
                button {
                    key: "{value}",
                    class: if selected == *value { "is-reader-sheet-chip is-reader-sheet-chip--active" } else { "is-reader-sheet-chip" },
                    role: "radio",
                    "aria-checked": selected == *value,
                    onclick: {
                        let value = value.to_string();
                        move |_| onpick.call(value.clone())
                    },
                    "{label}"
                }
            }
        }
    }
}
