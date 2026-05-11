//! Pill-shaped on/off toggle backed by `dioxus_primitives::switch::Switch`.
//!
//! Stateless wrapper — caller flips its own signal in `onchange`. The
//! primitive contributes `role="switch"`, `aria-checked`, and Space
//! handling; we layer the project's pill/knob styling on top via
//! `[data-state]` selectors in `main.css`.

use dioxus::prelude::*;
use dioxus_primitives::switch::{Switch, SwitchThumb};

#[component]
pub fn Toggle(on: bool, onchange: Option<EventHandler<bool>>) -> Element {
    rsx! {
        Switch {
            checked: on,
            on_checked_change: move |v: bool| {
                if let Some(handler) = &onchange {
                    handler.call(v);
                }
            },
            class: "settings-toggle",
            SwitchThumb { class: "settings-toggle-knob" }
        }
    }
}
