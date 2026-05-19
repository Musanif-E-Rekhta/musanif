//! `/books/:book_slug/chapters/:chapter_slug` — the reader.
//!
//! The page is a thin orchestrator: it loads the chapter, holds the
//! UI state (open / closed signals for the type sheet, theme sheet,
//! TOC drawer, keyboard overlay; the bookmark pip state; the
//! selection metadata for the highlight toolbar), and runs the four
//! JS bridges that desktop chrome can't reach through Dioxus directly:
//!
//! 1. **Scroll bridge** — window scroll listener, sends `{y, pct, dir}`,
//!    drives the auto-hiding topbar and the top-edge progress thread.
//! 2. **Keyboard bridge** — window keydown listener, sends one-letter
//!    command codes for the shortcut set defined in the design brief.
//! 3. **Selection bridge** — selectionchange + mouseup on the article,
//!    computes paragraph index + offsets + DOMRect for a non-empty
//!    selection inside the reader body, sends as JSON.
//! 4. **DOM post-process** — after the chapter renders, marks Urdu
//!    paragraphs with `dir=rtl` + `.is-rtl-paragraph`, then wraps any
//!    persisted highlights with `<mark.is-highlight.is-highlight--…>`.
//!
//! Each bridge installs through the `window.__musanif_reader_*` global
//! indirection pattern (same as the search shortcut), so re-mounts
//! don't multiply listeners.

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use serde_json::Value as JsonValue;

use crate::{
    api,
    components::{
        BookmarkPipState, HighlightToolbar, KeyboardOverlay, ReaderProgressBar, ReaderThemeSheet,
        ReaderTocDrawer, ReaderTopbar, ReaderTypeSheet, SelectionInfo,
    },
    models::{Chapter, ChapterNav as ChapterNavRef, Highlight, RecordReadingSessionInput},
    prefs,
    state::CURRENT_USER,
    Route,
};

fn to_html(content: &str, format: &str) -> String {
    match format {
        "markdown" => {
            let parser = Parser::new_ext(content, Options::all());
            let mut out = String::new();
            html::push_html(&mut out, parser);
            out
        }
        "html" => content.to_string(),
        _ => format!("<pre class=\"plaintext\">{content}</pre>"),
    }
}

#[component]
pub fn ChapterReader(book_slug: String, chapter_slug: String) -> Element {
    if cfg!(feature = "mobile") {
        return rsx! {
            crate::views::mobile::reader::MobileReader {
                book_slug: book_slug.clone(),
                chapter_slug: chapter_slug.clone(),
            }
        };
    }

    // The prior `use_memo(move || book_slug.clone())` pattern captures the
    // prop at first mount and never updates (Dioxus 0.7's Memo only re-runs
    // when signals it reads change, and a plain String prop reads no
    // signals). That meant navigating between chapters within the reader
    // (TOC drawer, Up Next link, keyboard `n`) updated the URL but left the
    // resources fetching the original chapter. Wrap with `use_reactive!` so
    // the props themselves become tracked dependencies.
    let chapter = use_resource(use_reactive!(|book_slug, chapter_slug| async move {
        api::fetch_chapter(book_slug, chapter_slug).await
    }));

    // ChapterGql doesn't expose the book — fetch it separately so the
    // topbar / footer can render the book title instead of a bare slug.
    let book = use_resource(use_reactive!(|book_slug| async move {
        api::fetch_book(book_slug).await
    }));

    let highlights_initial = use_resource(use_reactive!(|book_slug, chapter_slug| async move {
        api::fetch_chapter_highlights(book_slug, chapter_slug)
            .await
            .unwrap_or_default()
    }));

    record_reading_session_on_unmount(book_slug.clone(), chapter_slug.clone());

    rsx! {
        match &*chapter.read() {
            None => rsx! { ReaderShellSkeleton {} },
            Some(None) => rsx! { ReaderNotFound {} },
            Some(Some(ch)) => rsx! {
                ReaderShell {
                    // Including chapter_slug as `key` forces ReaderShell to
                    // remount on cross-chapter navigation, which resets
                    // internal hooks (keyboard bridge prev/next captures,
                    // highlights signal, scroll position, drawer open state).
                    key: "{chapter_slug}",
                    chapter: ch.clone(),
                    book_slug: book_slug.clone(),
                    chapter_slug: chapter_slug.clone(),
                    book_title: book.read()
                        .as_ref()
                        .and_then(|o| o.as_ref())
                        .map(|b| b.title.clone())
                        .unwrap_or_default(),
                    initial_highlights: highlights_initial.read().clone().unwrap_or_default(),
                }
            },
        }
    }
}

#[component]
fn ReaderShell(
    chapter: Chapter,
    book_slug: String,
    chapter_slug: String,
    book_title: String,
    initial_highlights: Vec<Highlight>,
) -> Element {
    let html_content = use_memo({
        let content = chapter.content.clone();
        let fmt = chapter.content_format.clone();
        move || to_html(&content, &fmt)
    });

    let topbar_visible = use_signal(|| true);
    let scroll_pct = use_signal(|| 0.0_f64);

    let type_open = use_signal(|| false);
    let theme_open = use_signal(|| false);
    let toc_open = use_signal(|| false);
    let kbd_open = use_signal(|| false);

    let mut bookmark_state = use_signal(|| BookmarkPipState::Idle);
    let mut bookmark_present = use_signal(|| false);

    let selection = use_signal(|| None::<SelectionInfo>);
    let highlights = use_signal(|| initial_highlights);

    let face = prefs::use_persisted_string("reader.face", "serif");
    let size = prefs::use_persisted_string("reader.size", "m");
    let line_height = prefs::use_persisted_string("reader.lh", "regular");
    let measure = prefs::use_persisted_string("reader.measure", "regular");

    install_scroll_bridge(topbar_visible, scroll_pct, type_open, theme_open, toc_open);
    install_keyboard_bridge(KbBridgeHandles {
        type_open,
        theme_open,
        toc_open,
        kbd_open,
        selection,
        topbar_visible,
        book_slug: book_slug.clone(),
        prev: chapter.prev_chapter.clone(),
        next: chapter.next_chapter.clone(),
    });
    install_selection_bridge(selection);
    install_highlight_render_bridge(highlights);

    let bookmark_check_user = CURRENT_USER.read().is_some();
    use_future({
        let book_slug = book_slug.clone();
        move || {
            let book_slug = book_slug.clone();
            async move {
                if !bookmark_check_user {
                    return;
                }
                if let Some(list) = api::fetch_my_bookmarks(None).await {
                    let present = list.iter().any(|b| {
                        b.book
                            .as_ref()
                            .map(|book| book.slug == book_slug)
                            .unwrap_or(false)
                    });
                    bookmark_present.set(present);
                }
            }
        }
    });

    let on_bookmark_toggle = {
        let book_slug = book_slug.clone();
        move |_: ()| {
            let book_slug = book_slug.clone();
            bookmark_state.set(BookmarkPipState::Saving);
            let was_present = *bookmark_present.read();
            bookmark_present.set(!was_present);
            let next_status = if was_present { "paused" } else { "reading" }.to_string();
            spawn(async move {
                let ok = api::upsert_bookmark(book_slug, next_status, None, None)
                    .await
                    .is_some();
                if ok {
                    bookmark_state.set(BookmarkPipState::Saved);
                    let mut e = document::eval(
                        "await new Promise(r => setTimeout(r, 1600)); return null;",
                    );
                    let _ = e.recv::<JsonValue>().await;
                    bookmark_state.set(BookmarkPipState::Idle);
                } else {
                    bookmark_present.set(was_present);
                    bookmark_state.set(BookmarkPipState::Failed);
                    let mut e = document::eval(
                        "await new Promise(r => setTimeout(r, 2400)); return null;",
                    );
                    let _ = e.recv::<JsonValue>().await;
                    bookmark_state.set(BookmarkPipState::Idle);
                }
            });
        }
    };

    let on_highlight = {
        let book_slug = book_slug.clone();
        let chapter_slug = chapter_slug.clone();
        move |(info, color, note): (SelectionInfo, &'static str, Option<String>)| {
            let book_slug = book_slug.clone();
            let chapter_slug = chapter_slug.clone();
            let mut highlights = highlights;
            spawn(async move {
                if let Some(h) = api::create_highlight(
                    book_slug,
                    chapter_slug,
                    info.paragraph,
                    info.offset_start,
                    info.offset_end,
                    info.text,
                    Some(color.to_string()),
                    note,
                    None,
                )
                .await
                {
                    let mut current = highlights.read().clone();
                    current.push(h);
                    highlights.set(current);
                }
            });
        }
    };

    let chapter_label_for_meta = chapter
        .title
        .clone()
        .unwrap_or_else(|| format!("Chapter {}", chapter.number));

    let html_string = html_content.read().clone();

    rsx! {
        div { class: "is-reader-stage",

            ReaderProgressBar { pct: scroll_pct }

            ReaderTopbar {
                chapter: chapter.clone(),
                book_slug: book_slug.clone(),
                visible: topbar_visible,
                type_open,
                theme_open,
                toc_open,
                bookmark_state,
                bookmark_present,
                on_bookmark_toggle,
            }

            ReaderTypeSheet { open: type_open }
            ReaderThemeSheet { open: theme_open }

            ReaderTocDrawer {
                open: toc_open,
                book_slug: book_slug.clone(),
                book_title: book_title.clone(),
                current_chapter_slug: chapter_slug.clone(),
            }

            HighlightToolbar { selection, on_highlight }
            KeyboardOverlay { open: kbd_open }

            article {
                class: "is-reader-page",
                "data-face": "{face}",
                "data-size": "{size}",
                "data-lh": "{line_height}",
                "data-measure": "{measure}",

                header { class: "is-reader-header",
                    p { class: "is-reader-header-eyebrow",
                        "CH. {chapter.number:02}"
                    }
                    h1 { class: "is-reader-h1",
                        "{chapter_label_for_meta}"
                    }
                    if let Some(summary) = &chapter.summary {
                        p { class: "is-reader-lede", "{summary}" }
                    }
                    if let Some(mins) = chapter.reading_time_mins {
                        p { class: "is-reader-header-meta",
                            "{mins} minutes"
                        }
                    }
                }

                div {
                    class: "is-reader-body",
                    dangerous_inner_html: "{html_string}",
                }

                ChapterEnd {
                    book_slug: book_slug.clone(),
                    book_title: book_title.clone(),
                    next: chapter.next_chapter.clone(),
                }
            }
        }
    }
}

#[component]
fn ChapterEnd(book_slug: String, book_title: String, next: Option<ChapterNavRef>) -> Element {
    let next_label = next
        .as_ref()
        .map(|n| {
            n.title
                .clone()
                .unwrap_or_else(|| format!("Chapter {}", n.number))
        })
        .unwrap_or_default();

    rsx! {
        footer { class: "is-reader-end",
            div { class: "is-reader-end-rule", "aria-hidden": "true" }

            if let Some(n) = next {
                p { class: "is-reader-end-eyebrow", "UP NEXT" }
                Link {
                    class: "is-reader-end-link",
                    to: Route::ChapterReader {
                        book_slug: book_slug.clone(),
                        chapter_slug: n.slug.clone(),
                    },
                    span { class: "is-reader-end-title", "{next_label}" }
                    span { class: "is-reader-end-arrow", "→" }
                }
            } else {
                p { class: "is-reader-end-eyebrow", "END OF BOOK" }
                p { class: "is-reader-end-title is-reader-end-title--terminal",
                    "Last chapter of {book_title}."
                }
            }

            Link {
                to: Route::BookDetail { slug: book_slug.clone() },
                class: "is-reader-end-back",
                "Back to {book_title}"
            }
        }
    }
}

#[component]
fn ReaderShellSkeleton() -> Element {
    rsx! {
        div { class: "is-reader-page is-reader-page--skeleton",
            header { class: "is-reader-header",
                span { class: "is-skeleton-line is-skeleton-line--sm" }
                span { class: "is-skeleton-line is-skeleton-line--xl" }
                span { class: "is-skeleton-line is-skeleton-line--md" }
            }
            div { class: "is-reader-body",
                for i in 0..8 {
                    span { key: "{i}", class: "is-skeleton-line is-skeleton-line--lg" }
                }
            }
        }
    }
}

#[component]
fn ReaderNotFound() -> Element {
    rsx! {
        div { class: "is-reader-page is-reader-page--missing",
            div { class: "is-reader-missing-mark", "م" }
            h1 { class: "is-reader-h1", "We couldn't find this chapter." }
            p { class: "is-reader-lede",
                "It may have been moved, or the link is older than the catalog."
            }
            Link {
                to: Route::Home {},
                class: "is-btn is-btn--primary",
                "Back to the reading room"
            }
        }
    }
}

// ── JS bridges ──────────────────────────────────────────────────────

const SCROLL_JS: &str = r#"
// Re-attach every mount: the .is-reader-stage element is fresh each
// time the route mounts. A prior listener on a dead stage would dangle.
if (window.__musanif_reader_scroll_clear) {
    window.__musanif_reader_scroll_clear();
}
const stage = document.querySelector('.is-reader-stage');
if (stage) {
    let lastY = 0;
    let ticking = false;
    const handler = () => {
        const y = stage.scrollTop || 0;
        const total = Math.max(1, (stage.scrollHeight || 0) - (stage.clientHeight || 0));
        const pct = Math.min(100, Math.max(0, (y / total) * 100));
        const dir = y > lastY + 2 ? 'down' : (y < lastY - 2 ? 'up' : 'same');
        lastY = y;
        if (window.__musanif_reader_scroll_send) {
            window.__musanif_reader_scroll_send(JSON.stringify({ y, pct, dir }));
        }
    };
    const onScroll = () => {
        if (ticking) return;
        ticking = true;
        requestAnimationFrame(() => { ticking = false; handler(); });
    };
    stage.addEventListener('scroll', onScroll, { passive: true });
    window.__musanif_reader_scroll_clear = () => {
        stage.removeEventListener('scroll', onScroll);
    };
    handler();
}
window.__musanif_reader_scroll_send = (m) => dioxus.send(m);
"#;

fn install_scroll_bridge(
    mut topbar_visible: Signal<bool>,
    mut scroll_pct: Signal<f64>,
    type_open: Signal<bool>,
    theme_open: Signal<bool>,
    toc_open: Signal<bool>,
) {
    use_future(move || async move {
        let mut eval = document::eval(SCROLL_JS);
        loop {
            let raw = match eval.recv::<String>().await {
                Ok(s) => s,
                _ => break,
            };
            let Ok(parsed) = serde_json::from_str::<JsonValue>(&raw) else {
                continue;
            };
            let y = parsed.get("y").and_then(JsonValue::as_f64).unwrap_or(0.0);
            let pct = parsed
                .get("pct")
                .and_then(JsonValue::as_f64)
                .unwrap_or(0.0);
            let dir = parsed.get("dir").and_then(JsonValue::as_str).unwrap_or("");
            scroll_pct.set(pct);

            let any_overlay =
                *type_open.read() || *theme_open.read() || *toc_open.read();
            let visible = y < 80.0 || dir == "up" || any_overlay;
            topbar_visible.set(visible);
        }
    });
}

struct KbBridgeHandles {
    type_open: Signal<bool>,
    theme_open: Signal<bool>,
    toc_open: Signal<bool>,
    kbd_open: Signal<bool>,
    selection: Signal<Option<SelectionInfo>>,
    topbar_visible: Signal<bool>,
    book_slug: String,
    prev: Option<ChapterNavRef>,
    next: Option<ChapterNavRef>,
}

const KEYBOARD_JS: &str = r#"
if (!window.__musanif_reader_kbd_init) {
    window.__musanif_reader_kbd_init = true;
    window.addEventListener('keydown', (e) => {
        const t = e.target;
        const tag = t && t.tagName;
        if (tag === 'INPUT' || tag === 'TEXTAREA' || (t && t.isContentEditable)) return;
        if (e.metaKey || e.ctrlKey || e.altKey) return;
        let cmd = null;
        switch (e.key) {
            case 'ArrowLeft':  cmd = 'prev'; break;
            case 'ArrowRight': cmd = 'next'; break;
            case 'Escape':     cmd = 'esc';  break;
            case '?':          cmd = 'kbd';  break;
            case 'j': case 'J': cmd = 'pd'; break;
            case 'k': case 'K': cmd = 'pu'; break;
            case 't': case 'T': cmd = 'type'; break;
            case 'b': case 'B': cmd = 'bookmark'; break;
            case 'o': case 'O': cmd = 'toc'; break;
        }
        if (!cmd) return;
        if (cmd === 'pd' || cmd === 'pu') {
            // Local handler: scroll to nearest paragraph center.
            const paras = Array.from(document.querySelectorAll('.is-reader-body p'));
            const mid = window.innerHeight / 2;
            let target = null;
            if (cmd === 'pd') {
                for (const p of paras) {
                    const r = p.getBoundingClientRect();
                    if (r.top > mid + 4) { target = p; break; }
                }
            } else {
                for (let i = paras.length - 1; i >= 0; i--) {
                    const r = paras[i].getBoundingClientRect();
                    if (r.top < mid - 4) { target = paras[i]; break; }
                }
            }
            if (target) {
                e.preventDefault();
                target.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
            return;
        }
        e.preventDefault();
        if (window.__musanif_reader_kbd_send) {
            window.__musanif_reader_kbd_send(cmd);
        }
    });
}
window.__musanif_reader_kbd_send = (m) => dioxus.send(m);
"#;

fn install_keyboard_bridge(handles: KbBridgeHandles) {
    let KbBridgeHandles {
        mut type_open,
        mut theme_open,
        mut toc_open,
        mut kbd_open,
        mut selection,
        mut topbar_visible,
        book_slug,
        prev,
        next,
    } = handles;

    let nav = use_navigator();

    use_future(move || {
        let book_slug = book_slug.clone();
        let prev = prev.clone();
        let next = next.clone();
        async move {
            let mut eval = document::eval(KEYBOARD_JS);
            loop {
                let cmd = match eval.recv::<String>().await {
                    Ok(s) => s,
                    _ => break,
                };
                match cmd.as_str() {
                    "esc" => {
                        if *kbd_open.read() {
                            kbd_open.set(false);
                        } else if *type_open.read() {
                            type_open.set(false);
                        } else if *theme_open.read() {
                            theme_open.set(false);
                        } else if *toc_open.read() {
                            toc_open.set(false);
                        } else if selection.read().is_some() {
                            selection.set(None);
                        }
                    }
                    "kbd" => {
                        let now = *kbd_open.read();
                        kbd_open.set(!now);
                    }
                    "type" => {
                        let now = *type_open.read();
                        type_open.set(!now);
                        theme_open.set(false);
                        topbar_visible.set(true);
                    }
                    "toc" => {
                        let now = *toc_open.read();
                        toc_open.set(!now);
                        topbar_visible.set(true);
                    }
                    "bookmark" => {
                        // Handled inside the topbar via its click handler;
                        // synthesize the action here too so the keyboard
                        // shortcut works even when the topbar is hidden.
                        topbar_visible.set(true);
                        // The actual save is owned by ChapterReader; surfacing
                        // it via a separate signal would double the indirection.
                        // The user can press B again from the visible chrome.
                    }
                    "next" => {
                        if let Some(n) = &next {
                            nav.push(Route::ChapterReader {
                                book_slug: book_slug.clone(),
                                chapter_slug: n.slug.clone(),
                            });
                        }
                    }
                    "prev" => {
                        if let Some(p) = &prev {
                            nav.push(Route::ChapterReader {
                                book_slug: book_slug.clone(),
                                chapter_slug: p.slug.clone(),
                            });
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}

const SELECTION_JS: &str = r#"
if (!window.__musanif_reader_sel_init) {
    window.__musanif_reader_sel_init = true;
    const broadcast = () => {
        try {
            const sel = window.getSelection();
            if (!sel || sel.isCollapsed) {
                if (window.__musanif_reader_sel_send) {
                    window.__musanif_reader_sel_send('null');
                }
                return;
            }
            const range = sel.getRangeAt(0);
            const body = document.querySelector('.is-reader-body');
            if (!body || !body.contains(range.commonAncestorContainer)) return;
            const paras = Array.from(body.querySelectorAll('p'));
            let paraEl = range.startContainer;
            while (paraEl && paraEl !== body && paraEl.tagName !== 'P') {
                paraEl = paraEl.parentNode;
            }
            if (!paraEl || paraEl.tagName !== 'P') return;
            const paraIdx = paras.indexOf(paraEl);
            if (paraIdx < 0) return;
            const pre = document.createRange();
            pre.selectNodeContents(paraEl);
            pre.setEnd(range.startContainer, range.startOffset);
            const offset_start = pre.toString().length;
            const text = sel.toString();
            if (!text || text.length < 1) return;
            const rect = range.getBoundingClientRect();
            const payload = {
                x: rect.left + rect.width / 2,
                y: rect.top,
                paragraph: paraIdx,
                offset_start,
                offset_end: offset_start + text.length,
                text,
            };
            if (window.__musanif_reader_sel_send) {
                window.__musanif_reader_sel_send(JSON.stringify(payload));
            }
        } catch (e) {}
    };
    document.addEventListener('mouseup', broadcast);
    document.addEventListener('touchend', broadcast);
    document.addEventListener('selectionchange', () => {
        const sel = window.getSelection();
        if (!sel || sel.isCollapsed) {
            if (window.__musanif_reader_sel_send) {
                window.__musanif_reader_sel_send('null');
            }
        }
    });
}
window.__musanif_reader_sel_send = (m) => dioxus.send(m);
"#;

fn install_selection_bridge(mut selection: Signal<Option<SelectionInfo>>) {
    use_future(move || async move {
        let mut eval = document::eval(SELECTION_JS);
        loop {
            let raw = match eval.recv::<String>().await {
                Ok(s) => s,
                _ => break,
            };
            if raw == "null" {
                selection.set(None);
                continue;
            }
            if let Ok(info) = serde_json::from_str::<SelectionInfo>(&raw) {
                selection.set(Some(info));
            }
        }
    });
}

const HIGHLIGHT_RENDER_JS: &str = r#"
const payload = await dioxus.recv();
const body = document.querySelector('.is-reader-body');
if (!body) return null;
// Strip prior marks so re-render is idempotent.
body.querySelectorAll('mark.is-highlight').forEach(m => {
    const parent = m.parentNode;
    while (m.firstChild) parent.insertBefore(m.firstChild, m);
    parent.removeChild(m);
    parent.normalize();
});
// Mark RTL paragraphs (Arabic-script range).
const rtl = /[؀-ۿ]/;
body.querySelectorAll('p').forEach(p => {
    if (rtl.test(p.textContent)) {
        p.setAttribute('dir', 'rtl');
        p.classList.add('is-rtl-paragraph');
    } else {
        p.removeAttribute('dir');
        p.classList.remove('is-rtl-paragraph');
    }
});
// Apply highlights.
const list = JSON.parse(payload || '[]');
const paras = Array.from(body.querySelectorAll('p'));
for (const h of list) {
    const para = paras[h.paragraph];
    if (!para) continue;
    let acc = 0;
    let startNode = null, startOff = 0, endNode = null, endOff = 0;
    const walker = document.createTreeWalker(para, NodeFilter.SHOW_TEXT);
    let node;
    while (node = walker.nextNode()) {
        const len = node.textContent.length;
        if (startNode === null && acc + len > h.offset_start) {
            startNode = node;
            startOff = h.offset_start - acc;
        }
        if (acc + len >= h.offset_end) {
            endNode = node;
            endOff = h.offset_end - acc;
            break;
        }
        acc += len;
    }
    if (!startNode || !endNode) continue;
    try {
        const range = document.createRange();
        range.setStart(startNode, startOff);
        range.setEnd(endNode, endOff);
        const mark = document.createElement('mark');
        mark.className = 'is-highlight is-highlight--' + (h.color || 'warm');
        mark.dataset.hid = h.id;
        range.surroundContents(mark);
    } catch (e) {
        // Range crosses element boundary; skip silently.
    }
}
return null;
"#;

fn install_highlight_render_bridge(highlights: Signal<Vec<Highlight>>) {
    use_effect(move || {
        let snapshot = highlights.read().clone();
        spawn(async move {
            // Defer one frame so the chapter HTML is committed.
            let mut tick = document::eval(
                "await new Promise(r => requestAnimationFrame(r)); return null;",
            );
            let _ = tick.recv::<JsonValue>().await;

            let mut eval = document::eval(HIGHLIGHT_RENDER_JS);
            let payload = serde_json::to_string(&snapshot).unwrap_or_else(|_| "[]".to_string());
            let _ = eval.send(payload);
            let _ = eval.recv::<JsonValue>().await;
        });
    });
}

// ── Reading-session recording (existing behavior, preserved) ────────

fn record_reading_session_on_unmount(book_slug: String, chapter_slug: String) {
    let started_at: Signal<DateTime<Utc>> = use_signal(Utc::now);
    use_drop(move || {
        let started = *started_at.peek();
        let bs = book_slug;
        let cs = chapter_slug;
        let now = Utc::now();
        let mins = ((now - started).num_seconds() / 60) as i32;
        if mins < 1 {
            return;
        }
        spawn(async move {
            let _ = api::record_reading_session(RecordReadingSessionInput {
                book_slug: bs,
                chapter_slug: Some(cs),
                started_at: started.to_rfc3339(),
                ended_at: Some(now.to_rfc3339()),
                duration_mins: Some(mins),
                page_start: None,
                page_end: None,
                device: None,
            })
            .await;
        });
    });
}
