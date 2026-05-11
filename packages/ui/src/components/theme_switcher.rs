use crate::{theme, Theme, CURRENT_THEME};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut open = use_signal(|| false);

    let themes = [
        (Theme::Parchment, "Parchment", "#f4ede1", "#b8412f"),
        (Theme::Midnight, "Midnight", "#0f1216", "#ec7666"),
        (Theme::SepiaDark, "Sepia Dark", "#1c1610", "#ec8775"),
        (Theme::Ink, "Ink", "#000000", "#ed7561"),
    ];

    let is_dark = CURRENT_THEME() != Theme::Parchment;

    rsx! {
        div { class: "theme-fab-wrap",
            button {
                class: "theme-fab",
                onclick: move |_| open.toggle(),
                aria_label: "Theme",
                {if is_dark { "☾" } else { "☀" }}
            }
            if open() {
                div { class: "theme-panel",
                    for (t, name, bg, primary) in themes {
                        button {
                            key: "{name}",
                            class: if CURRENT_THEME() == t { "theme-option theme-option--active" } else { "theme-option" },
                            onclick: move |_| {
                                *CURRENT_THEME.write() = t;
                                open.set(false);
                                theme::apply_and_persist(t.as_str());
                            },
                            span { class: "theme-option-swatch", style: "background: {bg}",
                                span { class: "theme-option-dot", style: "background: {primary}" }
                            }
                            span { class: "theme-option-label", "{name}" }
                            if CURRENT_THEME() == t {
                                span { "✓" }
                            }
                        }
                    }
                }
            }
        }
    }
}
