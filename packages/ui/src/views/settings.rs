use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut active = use_signal(|| "reading");

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Settings" }
                span { class: "is-main-subtitle", "Configure your reading experience" }
            }

            div { class: "is-main-body", style: "padding: 0",
                div { style: "display: grid; grid-template-columns: 200px 1fr; min-height: 500px",
                    // Sub-nav
                    div { class: "settings-subnav",
                        for (id, label) in [("reading", "Reading"), ("display", "Display & theme"), ("account", "Account"), ("about", "About")] {
                            button {
                                key: "{id}",
                                class: if active() == id { "settings-subnav-btn settings-subnav-btn--active" } else { "settings-subnav-btn settings-subnav-btn--inactive" },
                                onclick: move |_| active.set(id),
                                "{label}"
                            }
                        }
                    }

                    // Content
                    div { class: "settings-content",
                        match active() {
                            "reading"  => rsx! { ReadingPrefs {} },
                            "display"  => rsx! { DisplayPrefs {} },
                            "account"  => rsx! { AccountPrefs {} },
                            _          => rsx! { AboutSection {} },
                        }
                    }
                }
            }
        }
    }
}

// ── Reading ───────────────────────────────────────────────────────

#[component]
fn ReadingPrefs() -> Element {
    let mut font_size = use_signal(|| 17u32);
    let mut line_height = use_signal(|| 175u32); // × 0.01 = 1.75
    let preview_text = "There is a way of waiting that does not look like waiting. The cup sits, and the night sits, and one is not aware that anything is being measured.";

    rsx! {
        div {
            h3 { style: "margin: 0 0 24px; font-size: 18px; font-weight: 700", "Reading preferences" }

            // Live preview
            div { style: "padding: 20px 24px; background: var(--bg-color); border-radius: 12px; margin-bottom: 24px; font-family: var(--font-serif); font-size: {font_size()}px; line-height: {line_height() as f32 / 100.0}; color: var(--text-main)",
                "{preview_text}"
            }

            label { class: "settings-field-label", "Font size · {font_size()}px" }
            input {
                r#type: "range", min: "14", max: "24", value: "{font_size()}",
                style: "width: 100%; accent-color: var(--primary); margin-bottom: 22px",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { font_size.set(v); }
                }
            }

            label { class: "settings-field-label", "Line height · {line_height() as f32 / 100.0:.2}" }
            input {
                r#type: "range", min: "140", max: "200", step: "5", value: "{line_height()}",
                style: "width: 100%; accent-color: var(--primary); margin-bottom: 22px",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { line_height.set(v); }
                }
            }

            label { class: "settings-field-label", "Dictionary" }
            p { class: "settings-field-hint", "Show word meanings on double-tap" }
            Toggle { on: true }
        }
    }
}

// ── Display ───────────────────────────────────────────────────────

#[component]
fn DisplayPrefs() -> Element {
    let themes = [
        (
            "parchment",
            "Parchment",
            "#f7f3ec",
            "#c24a3b",
            "Default warm light",
        ),
        ("midnight", "Midnight", "#14171c", "#e87060", "Cool dark"),
        (
            "sepia-dark",
            "Sepia Dark",
            "#1f1813",
            "#e8826f",
            "Warm dark",
        ),
        ("ink", "Ink", "#000000", "#ed7561", "Pure black, OLED"),
    ];
    let mut current_theme = use_signal(|| "parchment".to_string());

    rsx! {
        div {
            h3 { style: "margin: 0 0 24px; font-size: 18px; font-weight: 700", "Display & theme" }

            label { class: "settings-field-label", "Theme" }
            div { class: "settings-theme-grid", style: "margin-bottom: 22px",
                for (id, label, bg, primary, desc) in themes {
                    button {
                        key: "{id}",
                        class: if current_theme() == id { "settings-theme-btn settings-theme-btn--active" } else { "settings-theme-btn settings-theme-btn--inactive" },
                        onclick: {
                            let id = id.to_string();
                            move |_| {
                                current_theme.set(id.clone());
                                let _ = document::eval(&format!(
                                    "document.documentElement.setAttribute('data-theme', '{}'); \
                                     try {{ localStorage.setItem('musanif-theme', '{}'); }} catch(e) {{}}",
                                    id, id
                                ));
                            }
                        },
                        div { style: "width: 42px; height: 42px; border-radius: 10px; background: {bg}; border: 1px solid var(--border-light); position: relative; flex-shrink: 0",
                            span { style: "position: absolute; right: 4px; bottom: 4px; width: 12px; height: 12px; border-radius: 50%; background: {primary}; border: 2px solid var(--bg-card)" }
                        }
                        div { style: "flex: 1",
                            p { style: "margin: 0; font-size: 13px; font-weight: 600; color: var(--text-main)", "{label}" }
                            p { style: "margin: 0; font-size: 11px; color: var(--text-muted)", "{desc}" }
                        }
                        if current_theme() == id {
                            span { style: "color: var(--primary)", "✓" }
                        }
                    }
                }
            }

            label { class: "settings-field-label", "Auto-switch at sunset" }
            p { class: "settings-field-hint", style: "margin-bottom: 8px", "Use Midnight theme between 7pm and 7am" }
            Toggle { on: false }

            div { style: "margin-top: 22px" }
            label { class: "settings-field-label", "Show Urdu glyphs in UI" }
            p { class: "settings-field-hint", style: "margin-bottom: 8px", "Display book titles in original script alongside Roman" }
            Toggle { on: true }
        }
    }
}

// ── Account ───────────────────────────────────────────────────────

#[component]
fn AccountPrefs() -> Element {
    rsx! {
        div {
            h3 { style: "margin: 0 0 24px; font-size: 18px; font-weight: 700", "Account" }

            // User card
            div { style: "display: flex; gap: 14px; padding: 16px; background: var(--bg-color); border-radius: 12px; margin-bottom: 22px; align-items: center",
                div { style: "width: 52px; height: 52px; border-radius: 50%; background: var(--primary); color: var(--bg-card); display: flex; align-items: center; justify-content: center; font-size: 22px; font-weight: 700",
                    "G"
                }
                div { style: "flex: 1",
                    p { style: "margin: 0; font-size: 14px; font-weight: 700", "Guest" }
                    p { style: "margin: 0; font-size: 12px; color: var(--text-muted)", "Not signed in" }
                }
                button { class: "is-btn", "Sign In" }
            }

            // Plan
            label { class: "settings-field-label", "Plan" }
            div { style: "padding: 14px; background: var(--bg-color); border-radius: 10px; display: flex; justify-content: space-between; align-items: center; margin-bottom: 22px",
                div {
                    p { style: "margin: 0; font-size: 13px; font-weight: 600", "Musanif Reader" }
                    p { style: "margin: 0; font-size: 11px; color: var(--text-muted)", "Free" }
                }
            }

            label { class: "settings-field-label", "Two-factor authentication" }
            p { class: "settings-field-hint", style: "margin-bottom: 8px", "Extra security on sign-in" }
            Toggle { on: false }

            div { style: "margin-top: 30px; padding-top: 22px; border-top: 1px solid var(--border-light); display: flex; gap: 10px",
                button { class: "is-btn", "Sign out" }
                button { class: "is-btn", style: "color: #c2473d; border-color: #c2473d", "Delete account" }
            }
        }
    }
}

// ── About ─────────────────────────────────────────────────────────

#[component]
fn AboutSection() -> Element {
    rsx! {
        div {
            h3 { style: "margin: 0 0 24px; font-size: 18px; font-weight: 700", "About" }

            div { style: "padding: 22px; background: var(--bg-color); border-radius: 12px; text-align: center; margin-bottom: 22px",
                p { style: "font-family: var(--font-urdu); font-size: 36px; color: var(--primary); margin: 0 0 6px; line-height: 1",
                    "مصنف"
                }
                p { style: "margin: 0; font-family: var(--font-serif); font-size: 17px; font-weight: 600", "Musanif" }
                p { style: "margin: 4px 0 0; font-size: 12px; color: var(--text-muted)", "Version 0.7.1 · Build 2841" }
            }

            div { style: "display: flex; flex-direction: column; gap: 2px",
                for label in ["What's new", "Help & support", "Send feedback", "Privacy policy", "Terms of service"] {
                    button {
                        key: "{label}",
                        style: "display: flex; justify-content: space-between; align-items: center; padding: 12px 14px; background: transparent; border: none; border-bottom: 1px solid var(--border-light); color: var(--text-main); font-family: inherit; font-size: 13px; font-weight: 500; cursor: pointer; text-align: left; width: 100%",
                        span { "{label}" }
                        span { style: "color: var(--text-muted)", "›" }
                    }
                }
            }
        }
    }
}

// ── Toggle ────────────────────────────────────────────────────────

#[component]
fn Toggle(on: bool) -> Element {
    rsx! {
        button {
            class: if on { "settings-toggle settings-toggle--on" } else { "settings-toggle settings-toggle--off" },
            span {
                class: if on { "settings-toggle-knob settings-toggle-knob--on" } else { "settings-toggle-knob settings-toggle-knob--off" }
            }
        }
    }
}
