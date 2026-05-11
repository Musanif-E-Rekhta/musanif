use dioxus::prelude::*;

use crate::components::{Field, Toggle};
use crate::prefs;

#[component]
pub fn LibraryPrefs() -> Element {
    let mut sync_on_wifi = prefs::use_persisted_bool("library.sync_on_wifi", true);
    let mut auto_download = prefs::use_persisted_bool("library.auto_download", false);

    rsx! {
        div {
            h3 { class: "settings-h3", "Library" }

            Field {
                label: "Sync".to_string(),
                hint: Some("Keep bookmarks and highlights in sync across devices.".to_string()),
                Toggle {
                    on: *sync_on_wifi.read(),
                    onchange: move |v: bool| sync_on_wifi.set(v),
                }
            }

            Field {
                label: "Auto-download new chapters".to_string(),
                hint: Some("Cache the next chapter when you open a book on Wi-Fi.".to_string()),
                Toggle {
                    on: *auto_download.read(),
                    onchange: move |v: bool| auto_download.set(v),
                }
            }

            Field {
                label: "Storage".to_string(),
                hint: Some("Clearing the cache removes downloaded chapters; bookmarks stay.".to_string()),
                button { class: "is-btn", "Clear cached content" }
            }
        }
    }
}
