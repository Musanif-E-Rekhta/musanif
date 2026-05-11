use dioxus::prelude::*;

#[component]
pub fn AboutSection() -> Element {
    rsx! {
        div {
            h3 { class: "settings-h3", "About" }

            div { class: "about-hero",
                p { class: "about-mark", "مصنف" }
                p { class: "about-name", "Musanif" }
                p { class: "about-version", "Version 0.7.1 · Build 2841" }
            }

            div { class: "about-list",
                for label in ["What's new", "Help & support", "Send feedback", "Privacy policy", "Terms of service"] {
                    button {
                        key: "{label}",
                        class: "about-list-item",
                        span { "{label}" }
                        span { class: "about-list-arrow", "›" }
                    }
                }
            }
        }
    }
}
