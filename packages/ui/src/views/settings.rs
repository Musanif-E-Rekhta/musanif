use dioxus::prelude::*;

const SETTINGS_CSS: Asset = asset!("/assets/styling/settings.css");

#[component]
pub fn Settings() -> Element {
    let mut notifications = use_signal(|| true);
    let mut dark_mode = use_signal(|| false);

    rsx! {
        document::Link { rel: "stylesheet", href: SETTINGS_CSS }

        div { class: "settings-page",

            header { class: "settings-header",
                h1 { "Settings" }
            }

            // ── Preferences ──────────────────────────────────────────
            div { class: "settings-section",
                h2 { class: "settings-section-title", "Preferences" }

                div { class: "setting-item",
                    div { class: "setting-info",
                        span { class: "setting-label", "Dark Mode" }
                        span { class: "setting-desc", "Switch to a darker interface" }
                    }
                    label { class: "toggle",
                        input {
                            r#type: "checkbox",
                            checked: dark_mode(),
                            onchange: move |e| dark_mode.set(e.checked()),
                        }
                        span { class: "toggle-slider" }
                    }
                }

                div { class: "setting-item",
                    div { class: "setting-info",
                        span { class: "setting-label", "Notifications" }
                        span { class: "setting-desc", "Receive updates about new releases" }
                    }
                    label { class: "toggle",
                        input {
                            r#type: "checkbox",
                            checked: notifications(),
                            onchange: move |e| notifications.set(e.checked()),
                        }
                        span { class: "toggle-slider" }
                    }
                }
            }

            // ── Reading ──────────────────────────────────────────────
            div { class: "settings-section",
                h2 { class: "settings-section-title", "Reading" }

                div { class: "setting-item",
                    div { class: "setting-info",
                        span { class: "setting-label", "Font Size" }
                        span { class: "setting-desc", "Adjust text size for reading" }
                    }
                    select { class: "setting-select",
                        option { value: "sm", "Small" }
                        option { value: "md", selected: true, "Medium" }
                        option { value: "lg", "Large" }
                        option { value: "xl", "Extra Large" }
                    }
                }

                div { class: "setting-item",
                    div { class: "setting-info",
                        span { class: "setting-label", "Language" }
                        span { class: "setting-desc", "Preferred content language" }
                    }
                    select { class: "setting-select",
                        option { value: "ur", selected: true, "Urdu" }
                        option { value: "en", "English" }
                        option { value: "hi", "Hindi" }
                    }
                }
            }

            // ── About ────────────────────────────────────────────────
            div { class: "settings-section",
                h2 { class: "settings-section-title", "About" }
                p { class: "about-text", "مصنف v0.1.0" }
                p { class: "about-text about-text--muted", "Your digital Urdu library." }
            }
        }
    }
}
