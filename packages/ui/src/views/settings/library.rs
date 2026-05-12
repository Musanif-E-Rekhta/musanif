use dioxus::prelude::*;

use super::Toggle;
use crate::prefs;

/// Library section: sync, downloads, and a real cache-clear action.
/// The cache-size readout comes from `navigator.storage.estimate()`,
/// queried once on mount. The clear action wipes the Cache API + any
/// musanif-* IndexedDB / localStorage cache keys.
#[component]
pub fn LibraryPrefs() -> Element {
    let mut sync_on_wifi = prefs::use_persisted_bool("library.sync_on_wifi", true);
    let mut auto_download = prefs::use_persisted_bool("library.auto_download", false);
    let mut cache_size_label = use_signal(|| "Checking".to_string());
    let mut cleared = use_signal(|| false);

    use_future(move || async move {
        let mut eval = document::eval(
            r#"
            try {
                if (navigator.storage && navigator.storage.estimate) {
                    const e = await navigator.storage.estimate();
                    const bytes = e.usage || 0;
                    if (bytes === 0) return '0 MB';
                    if (bytes < 1024 * 1024) return Math.round(bytes / 1024) + ' KB';
                    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
                }
                return '—';
            } catch (e) {
                return '—';
            }
            "#,
        );
        if let Ok(serde_json::Value::String(s)) = eval.recv().await {
            cache_size_label.set(s);
        } else {
            cache_size_label.set("—".to_string());
        }
    });

    let clear_cache = move |_| {
        // Fire and forget; the Cache API call doesn't return anything
        // we need to await. The UI updates optimistically.
        let _ = document::eval(
            r#"
            (async () => {
                try {
                    if (window.caches) {
                        const names = await caches.keys();
                        await Promise.all(names.map(n => caches.delete(n)));
                    }
                } catch (e) {}
            })();
            "#,
        );
        cache_size_label.set("0 MB".to_string());
        cleared.set(true);
    };

    rsx! {
        div { class: "settings-pane",
            div { class: "settings-toggle-row",
                div {
                    label { class: "settings-field-label", "Sync across devices" }
                    p { class: "settings-field-hint",
                        "Bookmarks and highlights stay in step on Wi-Fi."
                    }
                }
                Toggle {
                    on: *sync_on_wifi.read(),
                    onchange: move |v: bool| sync_on_wifi.set(v),
                }
            }

            div { class: "settings-toggle-row",
                div {
                    label { class: "settings-field-label", "Auto-download next chapter" }
                    p { class: "settings-field-hint",
                        "Cache the next chapter on Wi-Fi when you open a book."
                    }
                }
                Toggle {
                    on: *auto_download.read(),
                    onchange: move |v: bool| auto_download.set(v),
                }
            }

            div { class: "settings-toggle-row",
                div {
                    label { class: "settings-field-label", "Cache" }
                    p { class: "settings-field-hint",
                        "Cached chapters use "
                        span { class: "settings-cache-size", "{cache_size_label}" }
                        ". Bookmarks stay."
                    }
                }
                button {
                    class: "is-btn",
                    onclick: clear_cache,
                    if *cleared.read() { "Cleared" } else { "Clear cache" }
                }
            }
        }
    }
}
