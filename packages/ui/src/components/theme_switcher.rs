use crate::{Theme, CURRENT_THEME};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let mut open = use_signal(|| false);

    let themes = [
        (Theme::Parchment, "Parchment", "#f7f3ec", "#c24a3b"),
        (Theme::Midnight, "Midnight", "#14171c", "#e87060"),
        (Theme::SepiaDark, "Sepia Dark", "#1f1813", "#e8826f"),
        (Theme::Ink, "Ink", "#000000", "#ed7561"),
    ];

    let is_dark = CURRENT_THEME() != Theme::Parchment;

    rsx! {
        div {
            style: "position: fixed; bottom: 20px; right: 20px; z-index: 200",
            button {
                onclick: move |_| open.toggle(),
                aria_label: "Theme",
                style: "width: 44px; height: 44px; border-radius: 50%;
                        background: var(--bg-card); border: 1px solid var(--border-light);
                        color: var(--text-main); cursor: pointer;
                        box-shadow: 0 4px 16px rgba(0,0,0,.18);
                        display: flex; align-items: center; justify-content: center; font-size: 18px",
                {if is_dark { "☾" } else { "☀" }}
            }
            if open() {
                div {
                    style: "position: absolute; bottom: 56px; right: 0;
                            background: var(--bg-card); border: 1px solid var(--border-light);
                            border-radius: 12px; padding: 10px; min-width: 200px;
                            box-shadow: 0 12px 32px rgba(0,0,0,.22);
                            display: flex; flex-direction: column; gap: 2px",
                    for (t , name , bg , primary) in themes {
                        button {
                            key: "{name}",
                            onclick: move |_| {
                                *CURRENT_THEME.write() = t;
                                open.set(false);
                                let theme_str = t.as_str();
                                let _ = document::eval(&format!(
                                    "document.documentElement.setAttribute('data-theme', '{theme_str}'); \
                                     try {{ localStorage.setItem('musanif-theme', '{theme_str}'); }} catch(e) {{}}"
                                ));
                            },
                            style: format_args!(
                                "display: flex; align-items: center; gap: 10px; \
                                 padding: 8px 10px; border-radius: 8px; cursor: pointer; \
                                 border: none; background: {}; \
                                 color: var(--text-main); font-family: inherit; font-size: 13px; font-weight: 500; \
                                 text-align: left",
                                if CURRENT_THEME() == t { "var(--accent-light)" } else { "transparent" }
                            ),
                            span {
                                style: "width: 22px; height: 22px; border-radius: 6px; background: {bg};
                                        border: 1px solid var(--border-light); position: relative",
                                span {
                                    style: "position: absolute; right: 2px; bottom: 2px; width: 8px; height: 8px;
                                            border-radius: 50%; background: {primary}; border: 1.5px solid var(--bg-card)"
                                }
                            }
                            span { style: "flex: 1", "{name}" }
                            if CURRENT_THEME() == t {
                                span { style: "color: var(--primary)", "✓" }
                            }
                        }
                    }
                }
            }
        }
    }
}
