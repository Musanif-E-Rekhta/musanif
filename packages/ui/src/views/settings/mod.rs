use dioxus::prelude::*;

mod about;
mod account;
mod display;
mod library;
mod reading;
mod toggle;

use about::AboutSection;
use account::AccountPrefs;
use display::DisplayPrefs;
use library::LibraryPrefs;
use reading::ReadingPrefs;
pub(super) use toggle::Toggle;

#[component]
pub fn Settings() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::settings::MobileSettings {} };
    }

    let mut active = use_signal(|| "reading");

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Settings" }
                span { class: "is-main-subtitle", "Configure your reading experience" }
            }

            div { class: "is-main-body is-main-body--flush",
                div { class: "settings-layout",
                    div { class: "settings-subnav",
                        for (id, label) in [
                            ("reading", "Reading"),
                            ("display", "Display & theme"),
                            ("library", "Library"),
                            ("account", "Account"),
                            ("about", "About"),
                        ] {
                            button {
                                key: "{id}",
                                class: if active() == id { "settings-subnav-btn settings-subnav-btn--active" } else { "settings-subnav-btn settings-subnav-btn--inactive" },
                                onclick: move |_| active.set(id),
                                "{label}"
                            }
                        }
                    }

                    div { class: "settings-content",
                        match active() {
                            "reading"  => rsx! { ReadingPrefs {} },
                            "display"  => rsx! { DisplayPrefs {} },
                            "library"  => rsx! { LibraryPrefs {} },
                            "account"  => rsx! { AccountPrefs {} },
                            _          => rsx! { AboutSection {} },
                        }
                    }
                }
            }
        }
    }
}
