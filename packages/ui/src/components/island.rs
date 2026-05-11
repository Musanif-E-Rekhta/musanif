use dioxus::prelude::*;

/// Card primitive — the rounded, shadowed surface every panel sits on.
///
/// `soft` switches to the subtle background, `quiet` removes border/shadow.
/// `class` is appended after `island ...` so callers can layer on `is-main`,
/// `is-nav`, `is-rail-card`, etc.
#[component]
pub fn Island(
    soft: Option<bool>,
    quiet: Option<bool>,
    class: Option<String>,
    children: Element,
) -> Element {
    let mut classes = String::from("island");
    if soft.unwrap_or(false) {
        classes.push_str(" island--soft");
    }
    if quiet.unwrap_or(false) {
        classes.push_str(" island--quiet");
    }
    if let Some(extra) = class.as_deref().filter(|c| !c.is_empty()) {
        classes.push(' ');
        classes.push_str(extra);
    }
    rsx! { div { class: "{classes}", {children} } }
}
