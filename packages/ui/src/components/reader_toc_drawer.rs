//! Slide-in chapter list drawer.
//!
//! Mounts off-screen, slides over the main column from the left when
//! `open` flips true. The reader behind it stays mounted but does not
//! receive scroll while the drawer is open (the drawer overlay catches
//! pointer events). Drawer closes on overlay click, Esc (handled by
//! the reader's keyboard bridge), or selecting a chapter row.
//!
//! Chapter rows lead with a small status mark: filled disc for the
//! current chapter, half-filled circle for read (placeholder: derived
//! from `is_published` until per-user progress lands), empty ring for
//! upcoming. The visual rhythm is a list of typeset rows, not a card
//! grid.

use dioxus::prelude::*;

use crate::{api, models::ChapterSummary, Route};

#[component]
pub fn ReaderTocDrawer(
    open: Signal<bool>,
    book_slug: String,
    book_title: String,
    current_chapter_slug: String,
) -> Element {
    let is_open = *open.read();
    let book_slug_for_fetch = book_slug.clone();
    let chapters = use_resource(move || {
        let bs = book_slug_for_fetch.clone();
        async move { api::fetch_chapters(bs).await }
    });

    // Close the drawer once the route push commits to a new chapter.
    // Keeping <Link> on the rows preserves middle-click and cmd-click
    // open-in-new-tab; deferring the close until the prop changes
    // sidesteps the Dioxus 0.7 Link/onclick unmount race.
    let mut close_signal = open;
    use_effect(use_reactive!(|current_chapter_slug| {
        let _ = current_chapter_slug;
        close_signal.set(false);
    }));

    let aside_class = if is_open {
        "is-reader-drawer is-reader-drawer--open"
    } else {
        "is-reader-drawer"
    };
    let overlay_class = if is_open {
        "is-reader-drawer-overlay is-reader-drawer-overlay--visible"
    } else {
        "is-reader-drawer-overlay"
    };

    rsx! {
        // The overlay is always in the tree so its fade transition runs
        // both in and out; it just goes pointer-events-none while closed.
        button {
            class: "{overlay_class}",
            "aria-hidden": !is_open,
            tabindex: -1,
            onclick: move |_| open.set(false),
        }

        aside {
            class: "{aside_class}",
            "aria-label": "Chapter list",
            "aria-hidden": !is_open,

            header { class: "is-reader-drawer-header",
                h2 { class: "is-reader-drawer-title", "{book_title}" }
            }

            div { class: "is-reader-drawer-body",
                match &*chapters.read() {
                    None => rsx! { DrawerSkeleton {} },
                    Some(None) => rsx! {
                        p { class: "is-reader-drawer-empty",
                            "We couldn't load the chapter list."
                        }
                    },
                    Some(Some(items)) if items.is_empty() => rsx! {
                        p { class: "is-reader-drawer-empty",
                            "This book has no chapters yet."
                        }
                    },
                    Some(Some(items)) => rsx! {
                        ChapterList {
                            chapters: items.clone(),
                            book_slug: book_slug.clone(),
                            current: current_chapter_slug.clone(),
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn ChapterList(
    chapters: Vec<ChapterSummary>,
    book_slug: String,
    current: String,
) -> Element {
    let current_index = chapters
        .iter()
        .position(|c| c.slug == current)
        .unwrap_or(0);

    rsx! {
        ol { class: "is-reader-drawer-list",
            role: "list",
            for (i, ch) in chapters.iter().enumerate() {
                {
                    let mark = if ch.slug == current {
                        "is-reader-drawer-mark is-reader-drawer-mark--current"
                    } else if i < current_index {
                        "is-reader-drawer-mark is-reader-drawer-mark--read"
                    } else {
                        "is-reader-drawer-mark is-reader-drawer-mark--upcoming"
                    };
                    let row_class = if ch.slug == current {
                        "is-reader-drawer-row is-reader-drawer-row--current"
                    } else {
                        "is-reader-drawer-row"
                    };
                    let title = ch
                        .title
                        .clone()
                        .unwrap_or_else(|| format!("Chapter {}", ch.number));

                    rsx! {
                        li { key: "{ch.id}",
                            Link {
                                class: "{row_class}",
                                to: Route::ChapterReader {
                                    book_slug: book_slug.clone(),
                                    chapter_slug: ch.slug.clone(),
                                },

                                span { class: "{mark}", "aria-hidden": "true" }

                                span { class: "is-reader-drawer-row-text",
                                    span { class: "is-reader-drawer-row-num", "{ch.number:02}" }
                                    span { class: "is-reader-drawer-row-title", "{title}" }
                                }

                                if let Some(mins) = ch.reading_time_mins {
                                    span { class: "is-reader-drawer-row-time", "{mins} min" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DrawerSkeleton() -> Element {
    rsx! {
        ol { class: "is-reader-drawer-list is-reader-drawer-list--skeleton",
            for i in 0..6 {
                li { key: "{i}", class: "is-reader-drawer-row",
                    span { class: "is-skeleton-line is-skeleton-line--md" }
                }
            }
        }
    }
}
