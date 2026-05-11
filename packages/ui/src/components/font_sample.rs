use dioxus::prelude::*;

/// One font-sample swatch — large glyph rendered in the target font with a
/// label underneath. Selectable by clicking; bound to the shared `active`
/// signal in the same way as `SegOption`.
#[component]
pub fn FontSample(
    value: String,
    label: String,
    sample: String,
    font_family: String,
    active: Signal<String>,
) -> Element {
    let mut active = active;
    let is_active = *active.read() == value;
    let class = if is_active {
        "font-sample font-sample--active"
    } else {
        "font-sample"
    };
    rsx! {
        button {
            class: "{class}",
            onclick: {
                let v = value.clone();
                move |_| active.set(v.clone())
            },
            div {
                class: "font-sample-glyph",
                style: "font-family: {font_family}",
                "{sample}"
            }
            div { class: "font-sample-label", "{label}" }
        }
    }
}
