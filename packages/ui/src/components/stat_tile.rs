use dioxus::prelude::*;

/// One cell of the 4-up stat grid (profile, admin dashboard).
#[component]
pub fn StatTile(value: String, label: String) -> Element {
    rsx! {
        div { class: "stat",
            p { class: "stat-value", "{value}" }
            p { class: "stat-label", "{label}" }
        }
    }
}

/// Wrap a list of `StatTile`s in a 4-column grid (collapses to 2 on narrow).
#[component]
pub fn StatGrid(children: Element) -> Element {
    rsx! { div { class: "stat-grid", {children} } }
}
