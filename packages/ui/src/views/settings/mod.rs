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

/// Single-scroll Settings page: each preference cluster is its own
/// section island, ordered by frequency-of-visit (Account first, About
/// last). The old Stripe-shaped left sub-nav is replaced by a tracked
/// chip row near the top that smooth-scrolls to a section via plain
/// hash links. Inline anchor navigation, no scroll-spy yet (open
/// question on the brief).
///
/// Why scroll instead of tabbed sub-nav: PRODUCT.md's "hospitality
/// before density" principle plus the brief's "anti-goal: wall of
/// toggles" both push toward content-first hierarchy, not nav-first.
/// Sections breathe through whitespace; users scroll to what they
/// want without losing context.
#[component]
pub fn Settings() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::settings::MobileSettings {} };
    }

    rsx! {
        div { class: "island is-main",
            div { class: "is-main-header",
                h2 { class: "is-main-title", "Settings" }
                span { class: "is-main-subtitle",
                    "Quiet adjustments to how Musanif feels and reads."
                }
            }

            div { class: "is-main-body settings-scroll",
                SettingsAnchorNav {}

                SettingsSection { id: "account", eyebrow: "ACCOUNT",
                    AccountPrefs {}
                }

                SettingsSection { id: "reading", eyebrow: "READING",
                    ReadingPrefs {}
                }

                SettingsSection { id: "display", eyebrow: "DISPLAY",
                    DisplayPrefs {}
                }

                SettingsSection { id: "library", eyebrow: "LIBRARY",
                    LibraryPrefs {}
                }

                SettingsSection { id: "about", eyebrow: "ABOUT",
                    AboutSection {}
                }
            }
        }
    }
}

#[component]
fn SettingsAnchorNav() -> Element {
    rsx! {
        nav {
            class: "settings-anchors",
            "aria-label": "Settings sections",
            SettingsAnchor { href: "#account", label: "Account" }
            SettingsAnchor { href: "#reading", label: "Reading" }
            SettingsAnchor { href: "#display", label: "Display" }
            SettingsAnchor { href: "#library", label: "Library" }
            SettingsAnchor { href: "#about", label: "About" }
        }
    }
}

#[component]
fn SettingsAnchor(href: &'static str, label: &'static str) -> Element {
    rsx! {
        a {
            class: "settings-anchor",
            href: "{href}",
            onclick: move |e| {
                e.prevent_default();
                let target = href.trim_start_matches('#');
                let _ = document::eval(&format!(
                    "const el = document.getElementById('{target}'); \
                     if (el) el.scrollIntoView({{ behavior: 'smooth', block: 'start' }});"
                ));
            },
            "{label}"
        }
    }
}

#[component]
fn SettingsSection(id: &'static str, eyebrow: &'static str, children: Element) -> Element {
    rsx! {
        section {
            id: "{id}",
            class: "settings-section-island",
            p { class: "settings-section-eyebrow", "{eyebrow}" }
            div { class: "settings-section-body", {children} }
        }
    }
}
