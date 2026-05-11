use dioxus::prelude::*;

/// One segmented-control button (e.g. typeface picker). Use multiple in a
/// flex row, each comparing against the same `active` signal.
#[component]
pub fn SegOption(
    value: String,
    active: Signal<String>,
    children: Element,
) -> Element {
    let mut active = active;
    let is_active = *active.read() == value;
    let class = if is_active {
        "seg-option seg-option--active"
    } else {
        "seg-option"
    };
    rsx! {
        button {
            class: "{class}",
            onclick: {
                let v = value.clone();
                move |_| active.set(v.clone())
            },
            {children}
        }
    }
}

/// Convenience layout — flex row, gap 6, wraps. Wrap a list of `SegOption`
/// in this so callers don't reach for inline styles.
#[component]
pub fn SegRow(children: Element) -> Element {
    rsx! { div { class: "seg-row", {children} } }
}
