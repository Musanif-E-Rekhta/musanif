use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdSearch, Icon};

use crate::{
    api,
    components::{Cover, FeatureCard, KbdHint, PageHeader, SearchPanel},
    curation::{self, Couplet},
    models::{Book, FeaturedBook},
    state::SEARCH_OPEN,
    Route,
};

/// Discover: the signed-in (or signed-out) home of the reader app.
///
/// The page reads top to bottom as a small editorial sequence rather
/// than a single uniform grid:
///
/// 1. A page header with title, subtitle, and a compact search trigger
///    (also keyed to ⌘K). The trigger opens [`SearchPanel`].
/// 2. A weekly-rotating Nastaliq couplet block — the page's signature.
///    Non-interactive; sets the room.
/// 3. A "Committed" treatment editor's-pick [`FeatureCard`].
/// 4. Up to two curated horizontal strips (ghazals, long-form) whose
///    contents are filtered from the catalog by hand-listed slugs in
///    [`curation`]. Strips silently drop out when nothing matches, so
///    a thin or shifting corpus never leaves empty rails.
/// 5. A single browseable grid under `THE CATALOG`.
///
/// The pre-corpus state — when the catalog returns zero books — gets
/// its own editorial seed treatment ("the shelves are being filled"),
/// not a generic empty box.
///
/// When the search panel is open, the panel takes over the top of the
/// main island and everything below it dims under a scrim.
#[component]
pub fn Home() -> Element {
    if cfg!(feature = "mobile") {
        return rsx! { crate::views::mobile::discover::MobileDiscover {} };
    }

    install_search_shortcut();

    // 48 is enough to populate two curated strips of up to 12 each
    // plus a healthy catalog grid below them without a second round
    // trip. Strips filter against this same list.
    let books =
        use_resource(move || async move { api::fetch_books(None, None, Some(48), None).await });
    let featured = use_resource(move || async move { api::fetch_featured().await });

    let search_open = *SEARCH_OPEN.read();
    let content_class = if search_open {
        "is-discover-content is-discover-content--dimmed"
    } else {
        "is-discover-content"
    };

    rsx! {
        div { class: "island is-main",
            PageHeader {
                title: "Discover".to_string(),
                subtitle: "Urdu literature, hosted with care".to_string(),
                actions: rsx! {
                    SearchTrigger {}
                },
            }

            div { class: "is-main-body",
                SearchPanel {}

                div {
                    class: "{content_class}",
                    aria_hidden: search_open,

                    if search_open {
                        button {
                            class: "is-discover-scrim",
                            "aria-label": "Close search",
                            onclick: move |_| *SEARCH_OPEN.write() = false,
                            tabindex: -1,
                        }
                    }

                    DiscoverBody { books, featured }
                }
            }
        }
    }
}

/// Sequences the editorial blocks. When the catalog is genuinely empty
/// (loaded successfully, zero rows), we replace the whole sequence
/// with a single seed state — there's no point rendering an Editor's
/// Pick skeleton or "browse all" eyebrow when there's nothing under
/// either of them.
#[component]
fn DiscoverBody(
    books: Resource<Option<Vec<Book>>>,
    featured: Resource<Option<FeaturedBook>>,
) -> Element {
    if matches!(&*books.read(), Some(Some(items)) if items.is_empty()) {
        return rsx! { EmptyCorpus {} };
    }

    rsx! {
        CoupletBlock {}

        FeaturedSlot { featured: featured.clone() }

        CuratedStrip {
            eyebrow: "GHAZALS · the form, the heart".to_string(),
            slugs: curation::STRIP_GHAZALS,
            books: books.clone(),
        }
        CuratedStrip {
            eyebrow: "LONG-FORM · for an evening".to_string(),
            slugs: curation::STRIP_LONG_FORM,
            books: books.clone(),
        }

        p { class: "section-head-eyebrow is-catalog-eyebrow", "BROWSE ALL" }

        BookGrid { books }
    }
}

#[component]
fn SearchTrigger() -> Element {
    rsx! {
        button {
            class: "is-search is-search--trigger",
            "aria-label": "Open search",
            "aria-haspopup": "dialog",
            onclick: move |_| *SEARCH_OPEN.write() = true,
            Icon {
                icon: LdSearch,
                width: 14,
                height: 14,
                class: "is-search-icon",
            }
            span { class: "is-search-trigger-placeholder",
                "Search books, poets, ghazals"
            }
            KbdHint { "⌘K" }
        }
    }
}

/// Install (once per process) a window-level ⌘K / Ctrl+K keydown
/// listener that toggles `SEARCH_OPEN`. Re-mounting Home rebinds the
/// active receiver via a global indirection (`window.__musanif_kbd_send`);
/// listeners installed by earlier mounts whose futures have been
/// dropped become no-ops, so the listener does not multiply across
/// navigations. ESC closing is handled locally inside the search panel
/// input so it doesn't intercept ESC on every other view.
fn install_search_shortcut() {
    use_future(move || async move {
        let mut eval = document::eval(
            r#"
            if (!window.__musanif_kbd_init) {
                window.__musanif_kbd_init = true;
                window.addEventListener('keydown', (e) => {
                    const k = e.key && e.key.toLowerCase();
                    if ((e.metaKey || e.ctrlKey) && k === 'k') {
                        e.preventDefault();
                        if (window.__musanif_kbd_send) {
                            window.__musanif_kbd_send('toggle');
                        }
                    }
                });
            }
            window.__musanif_kbd_send = (m) => dioxus.send(m);
            "#,
        );

        loop {
            match eval.recv::<String>().await {
                Ok(m) if m == "toggle" => {
                    let cur = *SEARCH_OPEN.read();
                    *SEARCH_OPEN.write() = !cur;
                }
                _ => break,
            }
        }
    });

    use_drop(|| {
        *SEARCH_OPEN.write() = false;
    });
}

/// Renders the weekly couplet. The Urdu lines lead in Nastaliq, right-
/// aligned RTL; a Roman transliteration sits underneath in serif italic
/// as a quiet aid for readers who can't parse the script yet. The
/// attribution is set in a smaller eyebrow.
#[component]
fn CoupletBlock() -> Element {
    let couplet: &'static Couplet = curation::weekly_couplet();

    rsx! {
        section { class: "is-couplet",
            "aria-label": "Couplet of the week",
            p { class: "is-couplet-eyebrow", "COUPLET OF THE WEEK" }
            div { class: "is-couplet-urdu",
                dir: "rtl",
                lang: "ur",
                p { class: "is-couplet-urdu-line", "{couplet.urdu_line_1}" }
                p { class: "is-couplet-urdu-line", "{couplet.urdu_line_2}" }
            }
            div { class: "is-couplet-roman",
                p { class: "is-couplet-roman-line", "{couplet.roman_line_1}" }
                p { class: "is-couplet-roman-line", "{couplet.roman_line_2}" }
            }
            p { class: "is-couplet-attribution", "— {couplet.attribution}" }
        }
    }
}

#[component]
fn FeaturedSlot(featured: Resource<Option<FeaturedBook>>) -> Element {
    // Three states:
    //   None              -> still loading; show skeleton
    //   Some(None)        -> loaded, backend has no featured book; show
    //                        nothing rather than perpetual skeleton
    //   Some(Some(pick))  -> render the FeatureCard
    let pick = match &*featured.read() {
        None => return rsx! { FeaturedSkeleton {} },
        Some(None) => return rsx! { Fragment {} },
        Some(Some(p)) => p.clone(),
    };

    let book = pick.book;
    let mono = book.title.chars().next().map(|c| c.to_string());
    let blurb = pick
        .blurb
        .or_else(|| book.description.clone())
        .or_else(|| book.summary.clone())
        .unwrap_or_else(|| "A new arrangement, freshly annotated.".to_string());
    let eyebrow = pick.eyebrow.unwrap_or_else(|| "Editor's Pick".to_string());
    let title = pick.headline.unwrap_or_else(|| book.title.clone());
    let slug = book.slug.clone();

    rsx! {
        FeatureCard {
            eyebrow,
            title,
            blurb,
            cover_urdu: Some(book.title.clone()),
            cover_mono: mono,
            actions: rsx! {
                Link {
                    to: Route::BookDetail { slug: slug.clone() },
                    class: "is-btn is-btn--primary",
                    "Start reading"
                }
                Link {
                    to: Route::BookDetail { slug },
                    class: "is-btn",
                    "Details"
                }
            },
        }
    }
}

#[component]
fn FeaturedSkeleton() -> Element {
    rsx! {
        div { class: "is-feature is-feature--skeleton",
            div { class: "is-feature-cover is-skeleton-block" }
            div {
                span { class: "is-skeleton-line is-skeleton-line--sm" }
                span { class: "is-skeleton-line is-skeleton-line--xl" }
                span { class: "is-skeleton-line is-skeleton-line--md" }
                span { class: "is-skeleton-line is-skeleton-line--md" }
            }
        }
    }
}

/// A horizontal scroll-snap strip whose contents are pulled from the
/// already-fetched catalog and filtered by a hand-listed slug array.
/// The strip renders only while books are loading (skeleton) or once
/// at least one slug matched — an empty result silently hides the
/// whole rail, including its eyebrow, so curation can drift ahead of
/// the corpus without leaving rails behind.
#[component]
fn CuratedStrip(
    eyebrow: String,
    slugs: &'static [&'static str],
    books: Resource<Option<Vec<Book>>>,
) -> Element {
    let books_read = books.read();
    let matched: Vec<Book> = match &*books_read {
        // Still loading — render a skeleton so the page doesn't pop
        // in twice (once for the grid, once for the strip).
        None => return rsx! { StripSkeleton { eyebrow } },
        Some(None) => return rsx! { Fragment {} },
        Some(Some(items)) => slugs
            .iter()
            .filter_map(|slug| items.iter().find(|b| b.slug == *slug).cloned())
            .collect(),
    };

    if matched.is_empty() {
        return rsx! { Fragment {} };
    }

    rsx! {
        section { class: "is-strip",
            p { class: "section-head-eyebrow is-strip-eyebrow", "{eyebrow}" }
            div { class: "is-strip-track",
                role: "list",
                for book in matched.iter() {
                    StripCard { key: "{book.id}", book: book.clone() }
                }
            }
        }
    }
}

#[component]
fn StripSkeleton(eyebrow: String) -> Element {
    rsx! {
        section { class: "is-strip is-strip--skeleton",
            p { class: "section-head-eyebrow is-strip-eyebrow", "{eyebrow}" }
            div { class: "is-strip-track",
                for i in 0..5 {
                    div { key: "{i}", class: "is-book is-book--skeleton is-strip-card",
                        div { class: "is-book-cover is-skeleton-block" }
                        span { class: "is-skeleton-line is-skeleton-line--md" }
                        span { class: "is-skeleton-line is-skeleton-line--sm" }
                    }
                }
            }
        }
    }
}

/// Card variant tuned for horizontal strips. Shares the `is-book`
/// affordances of the grid card (hover lift, cover shadow) but lives
/// in a flex track with fixed-width children and scroll-snap.
#[component]
fn StripCard(book: Book) -> Element {
    let author_names = book
        .authors
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|a| a.author.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let first_char = book.title.chars().next().unwrap_or('م').to_string();

    rsx! {
        Link {
            class: "is-book is-strip-card",
            to: Route::BookDetail { slug: book.slug.clone() },
            role: "listitem",

            div { class: "is-book-cover",
                Cover {
                    title: Some(book.title.clone()),
                    urdu: Some(book.title.clone()),
                    mono: Some(first_char),
                }
            }

            p { class: "is-book-meta", "{book.title}" }

            if !author_names.is_empty() {
                p { class: "is-book-author", "{author_names}" }
            }
        }
    }
}

#[component]
fn BookGrid(books: Resource<Option<Vec<Book>>>) -> Element {
    rsx! {
        match &*books.read() {
            None => rsx! { GridSkeleton {} },
            Some(None) => rsx! {
                div { class: "state-error",
                    p { "We can't reach the catalog right now." }
                    p { class: "state-error-hint",
                        "Make sure the backend is running on "
                        code { "localhost:9678" }
                        ", or try again in a moment."
                    }
                }
            },
            Some(Some(items)) if items.is_empty() => rsx! {
                // EmptyCorpus owns the zero-state at the page level;
                // BookGrid never reaches this branch in practice because
                // DiscoverBody short-circuits. Kept for the case where
                // BookGrid is used standalone.
                div { class: "state-empty",
                    "The corpus is quiet today. Come back soon."
                }
            },
            Some(Some(items)) => rsx! {
                div { class: "is-grid",
                    for book in items.iter() {
                        BookCard { key: "{book.id}", book: book.clone() }
                    }
                }
            },
        }
    }
}

#[component]
fn GridSkeleton() -> Element {
    rsx! {
        div { class: "is-grid is-grid--skeleton",
            for i in 0..8 {
                div { key: "{i}", class: "is-book is-book--skeleton",
                    div { class: "is-book-cover is-skeleton-block" }
                    span { class: "is-skeleton-line is-skeleton-line--md" }
                    span { class: "is-skeleton-line is-skeleton-line--sm" }
                }
            }
        }
    }
}

#[component]
fn BookCard(book: Book) -> Element {
    let author_names = book
        .authors
        .as_deref()
        .unwrap_or(&[])
        .iter()
        .map(|a| a.author.name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let first_char = book.title.chars().next().unwrap_or('م').to_string();

    rsx! {
        Link {
            class: "is-book",
            to: Route::BookDetail { slug: book.slug.clone() },

            div { class: "is-book-cover",
                Cover {
                    title: Some(book.title.clone()),
                    urdu: Some(book.title.clone()),
                    mono: Some(first_char),
                }
            }

            p { class: "is-book-meta", "{book.title}" }

            if !author_names.is_empty() {
                p { class: "is-book-author", "{author_names}" }
            }

            if let Some(rating) = book.avg_rating {
                span { class: "is-rating", "★ {rating:.1}" }
            }
        }
    }
}

/// Renders when the catalog has loaded and is empty. Replaces the whole
/// editorial sequence; we don't want skeletons, an Editor's-Pick rail,
/// or strip eyebrows around a void. The couplet still shows here because
/// it's the part of the page that doesn't depend on the corpus.
#[component]
fn EmptyCorpus() -> Element {
    rsx! {
        CoupletBlock {}

        section { class: "is-empty-corpus",
            p { class: "is-empty-corpus-eyebrow", "AT THE READING ROOM" }
            h2 { class: "is-empty-corpus-headline",
                "The shelves are being filled."
            }
            p { class: "is-empty-corpus-body",
                "Musanif is a reading room for Urdu literature, hosted with care. We're cataloguing the first volumes now — ghazals, long-form, essays — and you'll see them appear here as they're ready."
            }
            p { class: "is-empty-corpus-body is-empty-corpus-body--quiet",
                "Come back soon. There's nothing rushed about it."
            }
        }
    }
}
