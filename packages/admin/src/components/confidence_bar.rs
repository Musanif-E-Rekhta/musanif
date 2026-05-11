use dioxus::prelude::*;

/// Track + fill bar showing AI confidence on a 0..1 scale, color-coded by
/// the kit's three thresholds (≥85 ok, ≥65 warn, else bad).
#[component]
pub fn ConfidenceBar(value: f32) -> Element {
    let pct = (value * 100.0).round() as i32;
    let color = if value >= 0.85 {
        "var(--conf-ok)"
    } else if value >= 0.65 {
        "var(--conf-warn)"
    } else {
        "var(--conf-bad)"
    };
    rsx! {
        div { class: "adm-conf",
            div { class: "adm-conf-track",
                div {
                    class: "adm-conf-fill",
                    style: "width: {pct}%; background: {color}",
                }
            }
            div { class: "adm-conf-num", style: "color: {color}", "{pct}%" }
        }
    }
}
