use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowLeft, LdBell, LdBookmark, LdHardDrive, LdInfo, LdMail, LdShare2},
    Icon,
};

use crate::state::CURRENT_USER;
use crate::theme;

#[derive(Clone, Copy, PartialEq)]
struct ThemeOption {
    id: &'static str,
    label: &'static str,
    bg: &'static str,
    primary: &'static str,
}

const THEMES: &[ThemeOption] = &[
    ThemeOption { id: "parchment",  label: "Parchment",  bg: "#f7f3ec", primary: "#c24a3b" },
    ThemeOption { id: "midnight",   label: "Midnight",   bg: "#14171c", primary: "#e87060" },
    ThemeOption { id: "sepia-dark", label: "Sepia Dark", bg: "#1f1813", primary: "#e8826f" },
    ThemeOption { id: "ink",        label: "Ink",        bg: "#000000", primary: "#ed7561" },
];

#[component]
pub fn MobileSettings() -> Element {
    let mut size = use_signal(|| 17u32);
    let current_theme = use_signal(|| "parchment".to_string());

    let user = CURRENT_USER.read();

    rsx! {
        div { class: "is-mob-reader-chrome",
            button { class: "is-mob-reader-back",
                Icon { icon: LdArrowLeft, width: 16, height: 16 }
                "Back"
            }
            span {
                style: "position: absolute; left: 50%; transform: translateX(-50%); font-size: 15px; font-weight: 700",
                "Settings"
            }
            span {}
        }

        // Account card
        div {
            class: "island",
            style: "margin: 12px; padding: 16px; display: flex; gap: 12px; align-items: center",
            div {
                style: "width: 48px; height: 48px; border-radius: 50%; background: var(--primary); color: var(--bg-card); display: flex; align-items: center; justify-content: center; font-size: 20px; font-weight: 700",
                if let Some(u) = user.as_ref() {
                    "{u.username.chars().next().unwrap_or('?').to_ascii_uppercase()}"
                } else {
                    "G"
                }
            }
            div { style: "flex: 1",
                if let Some(u) = user.as_ref() {
                    p { style: "margin: 0; font-size: 15px; font-weight: 700", "{u.username}" }
                    p { style: "margin: 0; font-size: 12px; color: var(--text-muted)", "{u.email}" }
                } else {
                    p { style: "margin: 0; font-size: 15px; font-weight: 700", "Guest" }
                    p { style: "margin: 0; font-size: 12px; color: var(--text-muted)",
                        "Not signed in"
                    }
                }
            }
            span { style: "color: var(--text-muted)", "›" }
        }

        // Reading: font size live preview
        p { class: "is-mob-group-title", "Reading" }
        div { class: "is-mob-group", style: "padding: 16px",
            p {
                style: "margin: 0 0 10px; font-size: 11px; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.06em; font-weight: 700",
                "Font size · {size()}px"
            }
            input {
                r#type: "range",
                min: "14",
                max: "24",
                value: "{size()}",
                style: "width: 100%; accent-color: var(--primary)",
                oninput: move |e| {
                    if let Ok(v) = e.value().parse::<u32>() { size.set(v); }
                }
            }
            div {
                style: "margin-top: 14px; padding: 14px; background: var(--bg-color); border-radius: 8px; font-family: var(--font-serif, 'Crimson Pro', Georgia, serif); font-size: {size()}px; line-height: 1.6",
                "The night is long; even the moon is tired of waiting."
            }
        }

        // Theme picker
        p { class: "is-mob-group-title", "Theme" }
        div { class: "is-mob-group", style: "padding: 12px",
            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px",
                for t in THEMES {
                    ThemeButton {
                        key: "{t.id}",
                        opt: *t,
                        selected: *current_theme.read() == t.id,
                        onpick: {
                            let id = t.id;
                            let mut current_theme = current_theme;
                            move |_| {
                                current_theme.set(id.to_string());
                                theme::apply_and_persist(id);
                            }
                        },
                    }
                }
            }
        }

        // Library group
        p { class: "is-mob-group-title", "Library" }
        div { class: "is-mob-group",
            RowLink { icon: rsx! { Icon { icon: LdBookmark, width: 16, height: 16 } }, label: "Default sort", value: Some("Recent".to_string()) }
            RowLink { icon: rsx! { Icon { icon: LdShare2, width: 16, height: 16 } }, label: "Sync", value: Some("On".to_string()) }
            RowLink { icon: rsx! { Icon { icon: LdHardDrive, width: 16, height: 16 } }, label: "Storage", value: Some("1.2 GB".to_string()) }
        }

        // Account group
        p { class: "is-mob-group-title", "Account" }
        div { class: "is-mob-group",
            RowLink { icon: rsx! { Icon { icon: LdMail, width: 16, height: 16 } }, label: "Email", value: user.as_ref().map(|u| u.email.clone()) }
            RowLink { icon: rsx! { Icon { icon: LdBell, width: 16, height: 16 } }, label: "Notifications", value: None }
            RowLink { icon: rsx! { Icon { icon: LdInfo, width: 16, height: 16 } }, label: "Privacy", value: None }
        }

        p {
            style: "text-align: center; margin: 20px 0 30px; font-size: 11px; color: var(--text-muted); font-family: var(--font-urdu)",
            "مصنف · v0.4.2"
        }
    }
}

#[component]
fn ThemeButton(opt: ThemeOption, selected: bool, onpick: EventHandler<()>) -> Element {
    let border = if selected { "var(--primary)" } else { "transparent" };
    rsx! {
        button {
            onclick: move |_| onpick.call(()),
            style: "display: flex; align-items: center; gap: 8px; padding: 10px; background: var(--bg-color); border: 2px solid {border}; border-radius: 10px; cursor: pointer; font-family: inherit; text-align: left",
            div {
                style: "width: 26px; height: 26px; border-radius: 7px; background: {opt.bg}; border: 1px solid var(--border-light); position: relative; flex-shrink: 0",
                span {
                    style: "position: absolute; right: 2px; bottom: 2px; width: 8px; height: 8px; border-radius: 50%; background: {opt.primary}; border: 1.5px solid var(--bg-card)"
                }
            }
            span {
                style: "font-size: 12.5px; font-weight: 600; color: var(--text-main)",
                "{opt.label}"
            }
        }
    }
}

#[component]
fn RowLink(icon: Element, label: String, value: Option<String>) -> Element {
    rsx! {
        button { class: "is-mob-row-link",
            span { class: "is-mob-row-link-icon", {icon} }
            span { class: "is-mob-row-link-label", "{label}" }
            if let Some(v) = value.as_deref().filter(|v| !v.is_empty()) {
                span { class: "is-mob-row-link-value", "{v}" }
            }
            span { class: "is-mob-row-link-arrow", "›" }
        }
    }
}
