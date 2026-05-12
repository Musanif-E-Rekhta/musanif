//! Auto-hiding reader topbar.
//!
//! The bar collapses upward when the user has scrolled past 80px and is
//! travelling downward; it returns on any upward scroll, on focus into
//! one of its controls, or while any of its overlays (type / theme /
//! TOC) is open. The visibility signal is computed by the parent in
//! [`crate::views::chapter_reader`] and just consumed here as a prop.
//!
//! The bar hosts four affordances: a back-link to the book, a chapter
//! meta line, and four icon controls (Type, Theme, Bookmark, TOC). The
//! "More / ellipsis" button that lived here previously is gone: every
//! action it could host has been promoted to a real affordance.

use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdArrowLeft, LdBookmark, LdBookmarkCheck, LdList, LdMoon, LdType},
    Icon,
};

use crate::models::Chapter;
use crate::Route;

/// Tri-state for the bookmark button. `Saving` shows a subtle spinner
/// hint; `Saved` flashes the success chip for 1.6s then collapses back
/// to either `Idle` (no bookmark) or `Saved` persistent (filled icon).
#[derive(Clone, Copy, PartialEq)]
pub enum BookmarkPipState {
    Idle,
    Saving,
    Saved,
    Failed,
}

#[component]
pub fn ReaderTopbar(
    chapter: Chapter,
    book_slug: String,
    visible: Signal<bool>,
    type_open: Signal<bool>,
    theme_open: Signal<bool>,
    toc_open: Signal<bool>,
    bookmark_state: Signal<BookmarkPipState>,
    bookmark_present: Signal<bool>,
    on_bookmark_toggle: EventHandler<()>,
) -> Element {
    let visible_class = if *visible.read() {
        "is-reader-topbar is-reader-topbar--visible"
    } else {
        "is-reader-topbar is-reader-topbar--hidden"
    };

    let book_title = chapter
        .book
        .as_ref()
        .map(|b| b.title.clone())
        .unwrap_or_default();

    let chapter_label = chapter
        .title
        .clone()
        .unwrap_or_else(|| format!("Chapter {}", chapter.number));

    let state = *bookmark_state.read();
    let has_bookmark = *bookmark_present.read();

    rsx! {
        header { class: "{visible_class}",
            role: "banner",
            "aria-hidden": !*visible.read(),

            Link {
                class: "is-reader-back",
                to: Route::BookDetail { slug: book_slug.clone() },
                "aria-label": "Back to book",
                Icon { icon: LdArrowLeft, width: 14, height: 14, class: "is-reader-back-icon" }
                span { class: "is-reader-back-title", "{book_title}" }
            }

            div { class: "is-reader-meta-line",
                span { class: "is-reader-meta-chapter", "{chapter_label}" }
                span { class: "is-reader-meta-dot", "·" }
                span { class: "is-reader-meta-time",
                    if let Some(mins) = chapter.reading_time_mins {
                        "{mins} min"
                    } else {
                        "—"
                    }
                }
                if state == BookmarkPipState::Saved {
                    span { class: "is-reader-saved-chip", "Saved" }
                }
                if state == BookmarkPipState::Failed {
                    span { class: "is-reader-failed-chip", "Couldn't save" }
                }
            }

            nav { class: "is-reader-actions",
                "aria-label": "Reader controls",

                IconBtn {
                    label: "Typography".to_string(),
                    shortcut: "T".to_string(),
                    active: *type_open.read(),
                    onclick: move |_| {
                        let now = *type_open.read();
                        type_open.set(!now);
                        theme_open.set(false);
                    },
                    icon: rsx! { Icon { icon: LdType, width: 14, height: 14 } },
                }

                IconBtn {
                    label: "Theme".to_string(),
                    shortcut: String::new(),
                    active: *theme_open.read(),
                    onclick: move |_| {
                        let now = *theme_open.read();
                        theme_open.set(!now);
                        type_open.set(false);
                    },
                    icon: rsx! { Icon { icon: LdMoon, width: 14, height: 14 } },
                }

                IconBtn {
                    label: if has_bookmark { "Remove bookmark".to_string() } else { "Bookmark".to_string() },
                    shortcut: "B".to_string(),
                    active: has_bookmark,
                    onclick: move |_| on_bookmark_toggle.call(()),
                    icon: rsx! {
                        if has_bookmark {
                            Icon { icon: LdBookmarkCheck, width: 14, height: 14 }
                        } else {
                            Icon { icon: LdBookmark, width: 14, height: 14 }
                        }
                    },
                }

                IconBtn {
                    label: "Chapter list".to_string(),
                    shortcut: "O".to_string(),
                    active: *toc_open.read(),
                    onclick: move |_| {
                        let now = *toc_open.read();
                        toc_open.set(!now);
                    },
                    icon: rsx! { Icon { icon: LdList, width: 14, height: 14 } },
                }
            }
        }
    }
}

#[component]
fn IconBtn(
    label: String,
    shortcut: String,
    active: bool,
    onclick: EventHandler<MouseEvent>,
    icon: Element,
) -> Element {
    let class = if active {
        "is-reader-icon-btn is-reader-icon-btn--active"
    } else {
        "is-reader-icon-btn"
    };
    let aria_label = if shortcut.is_empty() {
        label.clone()
    } else {
        format!("{label} ({shortcut})")
    };

    rsx! {
        button {
            class: "{class}",
            "aria-label": "{aria_label}",
            "aria-pressed": active,
            onclick: move |e| onclick.call(e),
            {icon}
        }
    }
}
