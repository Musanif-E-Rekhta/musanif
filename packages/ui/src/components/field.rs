use dioxus::prelude::*;

/// Settings-style field wrapper: uppercase label, child control, optional
/// hint underneath. Kept as a primitive so reader settings, profile edit,
/// and admin forms share spacing/typography.
#[component]
pub fn Field(label: String, hint: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "settings-field",
            label { class: "settings-field-label", "{label}" }
            {children}
            if let Some(h) = hint.as_deref().filter(|s| !s.is_empty()) {
                p { class: "settings-field-hint", "{h}" }
            }
        }
    }
}
