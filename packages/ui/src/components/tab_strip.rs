use dioxus::prelude::*;

/// One pill in a `TabStrip`. `value` is the identifier the consumer compares
/// against; `count` is shown after a separator dot when present.
#[derive(Clone, PartialEq)]
pub struct Tab {
    pub value: String,
    pub label: String,
    pub count: Option<u32>,
}

impl Tab {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            count: None,
        }
    }
    pub fn with_count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }
}

/// Slack-style tab strip: row of pills under the page header. Active tab is
/// underlined in primary; inactive tabs are muted.
#[component]
pub fn TabStrip(tabs: Vec<Tab>, active: Signal<String>) -> Element {
    let mut active = active;
    rsx! {
        div { class: "tabs",
            for tab in tabs {
                button {
                    key: "{tab.value}",
                    class: if *active.read() == tab.value { "tab tab--active" } else { "tab" },
                    onclick: {
                        let value = tab.value.clone();
                        move |_| active.set(value.clone())
                    },
                    "{tab.label}"
                    if let Some(count) = tab.count {
                        " · {count}"
                    }
                }
            }
        }
    }
}
