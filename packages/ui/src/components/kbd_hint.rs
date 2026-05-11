use dioxus::prelude::*;

/// Small key-cap chip used inside search inputs and shortcut hints.
#[component]
pub fn KbdHint(children: Element) -> Element {
    rsx! { span { class: "is-kbd", {children} } }
}
