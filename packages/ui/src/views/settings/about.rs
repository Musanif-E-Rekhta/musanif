use dioxus::prelude::*;

/// About: stripped to bare minimum. Brand mark, manifesto pulled from
/// PRODUCT.md, version, license, credit. No fake links.
#[component]
pub fn AboutSection() -> Element {
    rsx! {
        div { class: "about-block",
            p { class: "about-mark", "مصنف" }
            p { class: "about-name", "Musanif" }
            p { class: "about-manifesto", "A reading room for Urdu literature." }

            div { class: "about-meta",
                p { class: "about-meta-row", "Version 0.7.1 · Build 2841" }
                p { class: "about-meta-row", "Released under CC0-1.0." }
                p { class: "about-meta-row about-meta-row--quiet", "Built by Usairim Isani." }
            }
        }
    }
}
