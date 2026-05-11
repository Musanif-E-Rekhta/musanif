use dioxus::prelude::*;

/// OS-agnostic tab-strip rendered *below* the desktop window chrome.
/// Used by the reader's "open books" strip and by admin's
/// `Admin · Library · Analytics` top-level tabs. Visual only — drag/close
/// behavior lives in the consuming binary.
#[component]
pub fn WindowTabStrip(children: Element) -> Element {
    rsx! { div { class: "dt-tabstrip", {children} } }
}

/// One pill in a `WindowTabStrip`. The `glyph` is an optional Urdu/admin
/// stamp shown on the left; `subtitle` shows below the title in a muted
/// color. `onclose` adds a `×` button that stops propagation.
#[component]
pub fn WindowTab(
    title: String,
    subtitle: Option<String>,
    glyph: Option<String>,
    active: bool,
    onclick: EventHandler<MouseEvent>,
    onclose: Option<EventHandler<MouseEvent>>,
) -> Element {
    let class = if active {
        "dt-tab dt-tab--active"
    } else {
        "dt-tab"
    };
    rsx! {
        button {
            class: "{class}",
            onmousedown: move |e| e.stop_propagation(),
            onclick: move |e| onclick.call(e),
            if let Some(g) = glyph.as_deref().filter(|s| !s.is_empty()) {
                span { class: "dt-tab-glyph", "{g}" }
            }
            span { class: "dt-tab-text",
                span { class: "dt-tab-title", "{title}" }
                if let Some(s) = subtitle.as_deref().filter(|s| !s.is_empty()) {
                    span { class: "dt-tab-subtitle", "{s}" }
                }
            }
            if let Some(handler) = onclose {
                span {
                    class: "dt-tab-close",
                    onmousedown: move |e| e.stop_propagation(),
                    onclick: move |e| {
                        e.stop_propagation();
                        handler.call(e);
                    },
                    "×"
                }
            }
        }
    }
}
