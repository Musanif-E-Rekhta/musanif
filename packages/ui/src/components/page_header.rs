use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use super::KbdHint;

/// Standard page header: title + optional subtitle on the left, actions on
/// the right. Slots for `title_slot` / `actions_slot` exist for views that
/// need a custom widget (e.g. a back button or a chips row) instead of plain
/// text.
#[component]
pub fn PageHeader(
    title: Option<String>,
    subtitle: Option<String>,
    title_slot: Option<Element>,
    actions: Option<Element>,
) -> Element {
    rsx! {
        div { class: "is-main-header",
            if let Some(slot) = title_slot {
                {slot}
            } else if let Some(t) = title {
                h2 { class: "is-main-title", "{t}" }
            }

            if let Some(s) = subtitle.as_deref().filter(|s| !s.is_empty()) {
                span { class: "is-main-subtitle", "{s}" }
            }

            if let Some(a) = actions {
                div { class: "is-main-actions", {a} }
            }
        }
    }
}

/// Search input used inside `PageHeader` actions. Pure UI — caller owns the
/// `value` signal and any submit/keystroke handling.
#[component]
pub fn SearchInput(
    placeholder: String,
    value: Signal<String>,
    show_kbd: Option<bool>,
) -> Element {
    let mut value = value;
    rsx! {
        div { class: "is-search",
            Icon { icon: LdSearch, width: 14, height: 14, class: "is-search-icon" }
            input {
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| value.set(e.value()),
            }
            if show_kbd.unwrap_or(false) {
                KbdHint { "⌘K" }
            }
        }
    }
}
