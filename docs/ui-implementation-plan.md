# UI Implementation Plan

> Goal: bring the four packages (`ui`, `web`, `desktop`, `mobile`, plus a new
> `admin`) up to parity with the Musanif Design System UI kits, while extracting
> every shared concern into the `ui` package.
>
> **Status:** plan only. No code changes yet — review and adjust before
> committing to any of this.

## 1. References

### 1.1 The kits we are matching

Living next to this repo at `../Musanif Design System/ui_kits/` (i.e.
`/home/usairim/Documents/projects/personal/Musanif-e-Rekhta/Musanif Design System/ui_kits/`):

| Kit                | Wraps                          | Primary novelty over what we have                                                                          |
| ------------------ | ------------------------------ | ---------------------------------------------------------------------------------------------------------- |
| `web/`             | The shared screen library      | Three-island Discover (nav · main · rail), feature card, ContinueRail with 3 stacked cards, kbd shortcut   |
| `desktop/`         | `web/` + frameless window chrome | Reference for the chapter-tab strip and active/inactive tab styling. **We do not adopt the macOS traffic-light dots** — the current `dt-window-chrome` (breadcrumb + min/max/close buttons) is OS-agnostic and stays. |
| `mobile/`          | `web/` styles, mobile screens  | Polished Discover (greeting, search, recommended carousel, highlights), reader chrome, profile, settings   |
| `admin/`           | `web/` + admin panel           | 5-stage book ingestion pipeline (Upload → Process → Review → Edit → Publish), provider/model picker        |

All four share the same theme tokens (`parchment` / `midnight` / `sepia-dark`
/ `ink`) and the `Cover`, `Island`, button, and chip primitives.

### 1.2 External design references the brief calls out

- **Slack** — compact density, sidebar grouped into named sections
  (Channels / DMs / Apps), small count badges, persistent left rail. Apply to
  the `is-nav` sidebar (Browse / Library / Collections groupings + count
  pills) and to the admin sidebar (Ingestion queue / Library / System).
- **Zen Browser** — floating island layout, generous radius (~14–18px), soft
  shadows, content rail on the right. Already the foundation of `is-shell`;
  push further by adopting the right rail on Discover and the focus-mode rail
  in the reader.
- **GitLab** — page header pattern (title + subtitle + actions on the right),
  data-dense tables with status pills, breadcrumbs in chrome. Apply to the
  admin "Recently uploaded" table, the book-header in admin, and the desktop
  breadcrumb (already present — extend with chapter-tab strip).

> **Desktop chrome is OS-agnostic by design.** The existing
> `dt-window-chrome` (Musanif breadcrumb · current-page · `— ❐ ✕`
> buttons) is what ships on every platform. We do **not** mimic macOS
> traffic-light dots even when the kit shows them; the kit's tab-strip
> *visuals* are still useful as a reference, but they live below or
> alongside our breadcrumb, not in place of it.

---

## 2. Current vs Kit — gap inventory

A side-by-side audit by view. ✅ already matches; ◐ partial; ❌ missing.

### 2.1 Shared primitives (`packages/ui/`)

| Concept                       | Kit class / component       | Today                              | Action                                                                  |
| ----------------------------- | --------------------------- | ---------------------------------- | ----------------------------------------------------------------------- |
| Theme tokens                  | 4 themes                    | ✅                                  | Verify HEX parity with the kit's palette and reconcile drift            |
| `Cover` placeholder           | `Cover` JSX                 | ✅ `components/cover.rs`            | Add an `accent` gradient prop (kit supports it)                         |
| Island base                   | `.island`                   | ✅ in `main.css`                    | None                                                                    |
| Theme FAB                     | `ThemePicker`               | ✅ `theme_switcher.rs`              | None                                                                    |
| Three-island shell            | `.is-shell` (3-col)         | ◐ only `--two`                     | Add 3-column grid + `is-shell--focus` (rail nav)                         |
| Right rail card               | `is-rail-card`              | ❌                                  | New                                                                     |
| Continue + Highlight + Goal   | `ContinueRail`              | ❌                                  | New `components/continue_rail.rs` (3 stacked islands)                   |
| Feature card                  | `is-feature`                | ❌                                  | New component or in `views/home.rs`                                     |
| Rail nav (focus mode)         | `RailIsland`                | ❌                                  | New `components/rail_island.rs`                                         |
| Chapter window-tab strip      | `is-win-tabs` (kit visual)  | ❌                                  | Desktop only — implemented as a strip *below* our existing `dt-window-chrome` (no traffic-light dots). See §3.3 |
| Stage stepper / pipeline rows | `adm-stepper`, `adm-pipeline` | ❌                                | Admin only — see §3.5                                                   |
| Confidence bar                | `adm-conf`                  | ❌                                  | Admin only                                                              |
| Diff view                     | `adm-diff`                  | ❌                                  | Admin only                                                              |
| Settings live preview         | `Field`, `SegOption`, font samples | ◐ basic                       | Extract `field.rs`, `seg_option.rs`, `font_sample.rs` into `ui/components` |

### 2.2 Reader app routes

| Route          | Kit                                         | Today (`packages/ui/src/views`)         | Gap                                                                              |
| -------------- | ------------------------------------------- | --------------------------------------- | -------------------------------------------------------------------------------- |
| `/` Discover   | feature card · search w/ ⌘K · `is-grid`     | search + grid                           | Add feature card; add ⌘K hint; add right-rail (ContinueRail) when 3-col layout active |
| `/books/:slug` | `is-detail` two-col, stats grid, TOC rows   | mostly there                            | Polish stat tiles; rating chip color; TOC time badge                             |
| Reader         | focus mode: rail nav + main only            | uses normal sidebar                     | Switch to `is-shell--focus` with `RailIsland`; richer header (type/moon icons)   |
| `/shelf`       | tab pills (Reading/Want/Finished/Notes)     | `tabs`/`tab--active`                    | Match label format `"Reading · 3"`; row-card styling                             |
| `/authors`     | grid of author cards w/ search bar          | grid w/o search                         | Add search header; refine card                                                   |
| `/authors/:slug` | hero + bibliography                       | OK                                      | Bring follow button; works grid uses Cover                                       |
| `/profile`     | stats(4) + goal + 2-col (Finished, Highlights) | barebones                            | Stats tiles; reading goal card; 2-col layout                                     |
| `/settings`    | 5 sub-sections (Reading, Display, Library, Account, About) | 4 (no Library)             | Add Library; richer Reading preview with font samples                            |
| `/login` `/signup` | shared shell                            | `AuthShell` already extracted           | None                                                                             |
| `/404`         | minimal                                     | OK                                      | None                                                                             |

### 2.3 Mobile

| Mobile screen     | Kit                                        | Today                | Gap                                              |
| ----------------- | ------------------------------------------ | -------------------- | ------------------------------------------------ |
| Discover          | greeting + search + Continue card + Recommended carousel + highlights | shell only | Build full screen                                |
| Library           | greeting + chip tabs + row list            | uses desktop view    | Build mobile-specific screen                     |
| Reader            | reader chrome + pill page indicator + action bar | uses desktop view  | Build mobile-specific screen                     |
| Authors / Profile / Settings | mobile-styled                  | uses desktop view    | Build mobile-specific screens                    |

### 2.4 Desktop

> Desktop is **OS-agnostic**: same chrome on Linux, Windows, macOS. We keep
> `dt-window-chrome` as-is and ignore the kit's macOS traffic-light dots.

| Concern              | Kit                                   | Today                           | Gap                                              |
| -------------------- | ------------------------------------- | ------------------------------- | ------------------------------------------------ |
| Window titlebar      | titlebar + traffic dots               | `dt-window-chrome` w/ breadcrumb + `— ❐ ✕` buttons | **Keep current.** No traffic-light dots. Polish: hover/active states, slightly larger hit area on the buttons |
| Chapter tab strip    | `is-win-tabs` row inside titlebar     | ❌                               | Add a *separate* `dt-tabstrip` row **below** the titlebar (or `dt-window-chrome` becomes a 2-row layout). Visually borrows the kit's tab styling but no integration with traffic lights |
| Window controls      | macOS dots                            | `— ❐ ✕` glyph buttons           | Keep current; ensure they read correctly across themes (especially Ink/Midnight) |
| Frameless resize     | n/a                                    | 8 hit-targets in `main.rs`      | Keep as is                                       |
| Drag region          | titlebar drags                        | titlebar drags                  | Tab strip should also be draggable in empty space (not on tab buttons) |

### 2.5 Admin (new)

Entire package missing. Five stage panels, sidebar queue, book header,
stepper. See §3.5.

---

## 3. Architecture: where each piece lives

The principle is the one we already used in the API/Graphql split: anything
that more than one binary needs lives in `ui`. Only the *shells* belong in
the platform crates.

```
packages/ui/                          ← shared
  src/
    components/
      cover.rs                        ← (existing)
      theme_switcher.rs               ← (existing)
      auth_form.rs                    ← (existing)
      island.rs                       ← NEW: Island { children, soft?, quiet? }
      nav_island.rs                   ← NEW: full sidebar (Browse/Library/Collections)
      rail_island.rs                  ← NEW: focus-mode collapsed nav
      continue_rail.rs                ← NEW: ContinueRail wrapping 3 cards
      continue_card.rs                ← NEW
      highlight_card.rs               ← NEW
      reading_goal_card.rs            ← NEW
      feature_card.rs                 ← NEW
      stat_tile.rs                    ← NEW (used in profile + admin)
      page_header.rs                  ← NEW: title + subtitle + actions slot
      tab_strip.rs                    ← NEW: Slack-style tab pills used by Shelf
      window_tabstrip.rs              ← NEW: OS-agnostic chapter-tab row for desktop chrome (reused by admin)
      kbd_hint.rs                     ← NEW: ⌘K-style key chip
    views/                            ← reader-app screens (already exist)
    views/admin/                      ← NEW (see §3.5)
    state.rs, theme.rs, routes.rs     ← (existing)
    api/, graphql/, models.rs         ← (existing, untouched)

packages/web/        ← thin: theme apply, router, no chrome
packages/desktop/    ← window chrome + chapter-tab strip + drag/resize
packages/mobile/     ← MobileShell + bottom-tabs + per-screen mobile views
packages/admin/      ← NEW (separate binary, reuses ui crate)
```

### 3.1 Why this split

- **Reader views** stay in `ui::views::*`. Web/desktop/mobile crates render
  the same `Route` tree.
- **Mobile screens** that diverge from desktop (different chrome, different
  density) live in `ui::views::mobile::*` behind the `mobile` feature flag,
  and `MobileShell` swaps them in via `cfg!(feature = "mobile")` in the
  layout (already the pattern).
- **Admin** is a separate crate so its routes, sidebar, and dependencies
  don't pollute the reader bundle. It still re-uses `ui::components::*`,
  `ui::theme`, `ui::state`, `ui::api`, `ui::graphql`, `ui::config`.
- **Window chrome** lives in `desktop/` because only Tao gives us
  `drag_resize_window`. It is OS-agnostic — same `dt-window-chrome`
  rendered on every platform, no platform-specific styling. The
  chapter-tab strip's *visual* lives in `ui` (so admin can reuse it for
  "Admin · Library · Analytics" tabs); the *behavior* (close/maximize/
  minimize handlers, drag, drag-resize) stays in `desktop/`.

### 3.2 Dioxus components we should lean on

The `ui` package already uses `dioxus-free-icons` (Lucide) — keep using it.
For new components we need:

- `Link` from `dioxus_router` — existing (used).
- `Outlet::<Route>` — existing.
- `use_signal`, `use_resource`, `use_memo`, `use_effect` — existing.
- `document::eval` — existing for the localStorage one-liners.
- **No third-party Dioxus component crates needed** — the design is built
  from primitives. Avoid adding `dioxus-material` / `dioxus-ui` etc.; the
  bespoke islands are part of the brand.

### 3.3 Desktop chapter tabs

The kit shows tabs like `Diwan-e-Ghalib · Bang-e-Dara · Aag Ka Darya`
**inside** a macOS-style titlebar with traffic-light dots. We **don't** copy
that integration — instead, the tab strip lives as a *separate row* directly
beneath our existing `dt-window-chrome`:

```
┌────────────────────────────────────────────────────────────┐
│  Musanif › Reading                                  — ❐ ✕  │  ← dt-window-chrome (unchanged)
├────────────────────────────────────────────────────────────┤
│  [م Diwan-e-Ghalib] [م Bang-e-Dara · Ch. 14] [م Aag…]      │  ← NEW dt-tabstrip
├────────────────────────────────────────────────────────────┤
│  ... is-shell ...                                          │
└────────────────────────────────────────────────────────────┘
```

This keeps the chrome OS-agnostic and means the same layout works on Linux,
Windows, and macOS without conditional rendering.

We have no concept of "open books" yet. Two options for what populates the
strip:

1. **Recent books proxy** — populate the strip from the user's last-N
   bookmarks with `status="reading"`. Lowest effort, immediately useful.
2. **True multi-tab reading** — a `OPEN_BOOKS` global signal (`Vec<Book>`)
   plus open/close handlers. Closer to the design but is its own feature.

Recommend starting with (1) and treating (2) as a follow-up. The strip is
also the natural extension point for admin's `Admin · Library · Analytics`
top-level tabs — same `dt-tabstrip` component, different content.

### 3.4 Reading goal data

The Discover/Profile rail shows `14 / 24 books in 2026`. Backend already
exposes `fetch_my_reading_goal` (REST + GQL). Wire that up; don't hardcode.

### 3.5 Admin package — folder layout

```
packages/admin/
  Cargo.toml
  assets/
    main.css
    admin.css                ← lifted from ui_kits/admin/admin.css
  src/
    main.rs                  ← Dioxus desktop+web entry; window chrome
    routes.rs                ← /, /jobs/:id, /library, /analytics
    state.rs                 ← CURRENT_JOB, JOB_FILTER, etc.
    components/
      sidebar.rs             ← queue list + library + system + user footer
      stage_stepper.rs       ← StageStepper (1..=5)
      mini_cover.rs          ← thin wrapper around ui::components::Cover
      confidence_bar.rs
      book_queue_item.rs
      book_header.rs
      pipeline_row.rs
      live_log.rs
      provider_picker.rs
      cost_card.rs
      diff_view.rs
      visibility_radio.rs
      checklist.rs
      ai_metadata.rs         ← summary, entities, themes, suggested actions
    views/
      upload.rs              ← drop-zone + recent-uploads table
      process.rs              ← pipeline + log + provider + cost
      review.rs               ← PDF preview · chapter list · AI metadata
      edit.rs                 ← AI vs human diff
      publish.rs              ← preview card + visibility + checklist
      mock_data.rs            ← QUEUE, CHAPTERS, PROCESS_LOG (mirrors kit)
```

Backend questions (PDF parse, OCR, AI, embeddings, job queue) are out of
scope for this UI plan — see §6.

---

## 4. Phased rollout

Each phase ends with a green `cargo check --workspace`. Numbers in `[]` are
rough sequencing dependencies.

### Phase A · Shared primitives in `ui` (no new screens yet)

- A1 Add `is-shell` 3-col grid and `is-shell--focus` rail variant to
  `ui/assets/styling/main.css` (or split into `shell.css`).
- A2 `components/island.rs` (a thin wrapper, useful for prop-driven variants).
- A3 `components/page_header.rs` — replaces inline `is-main-header` in 6
  views.
- A4 `components/feature_card.rs`.
- A5 `components/continue_card.rs`, `highlight_card.rs`,
  `reading_goal_card.rs`, `continue_rail.rs`.
- A6 `components/rail_island.rs`.
- A7 `components/nav_island.rs` — replaces hand-rolled JSX in `navbar.rs`
  (Slack-grouping: Browse · Library · Collections).
- A8 `components/stat_tile.rs`, `tab_strip.rs`, `kbd_hint.rs`.
- A9 Reconcile theme tokens vs kit (audit `main.css` § DESIGN TOKENS).

### Phase B · Reader-app screens to parity

- B1 Discover: 3-column on desktop with `ContinueRail`; feature card; ⌘K
  search hint. Wire feature card to `fetch_books` "editor's pick"
  placeholder (or pin a book by slug).  [needs A1, A4, A5]
- B2 Book detail: stat tiles, rating chip color, TOC time badge.  [A3, A8]
- B3 Reader: `is-shell--focus` + `RailIsland`; new icon header (type, moon,
  bookmark, more).  [A1, A6]
- B4 Shelf: pill-tab strip, row-card layout.  [A8]
- B5 Authors: search bar in header; refined card.  [A3]
- B6 Profile: stats tiles · goal card · 2-col Finished + Highlights.
  [A5, A8]
- B7 Settings: add Library section; Reading-section live preview with
  font-family swatches; ports kit's Field/SegOption.  [A8]

### Phase C · Mobile screens

- C1 `views/mobile/discover.rs` — greeting, search, Continue card,
  Recommended carousel, highlights island.
- C2 `views/mobile/library.rs` — chip tabs + row list.
- C3 `views/mobile/reader.rs` — reader chrome + page-pill + action bar.
- C4 `views/mobile/authors.rs`.
- C5 `views/mobile/profile.rs`.
- C6 `views/mobile/settings.rs`.
- C7 Update `mobile_shell.rs` to route to mobile-specific views when
  `feature = "mobile"`.

### Phase D · Desktop chrome (OS-agnostic — keep current titlebar)

- D1 Build a new `dt-tabstrip` row below `dt-window-chrome` (separate
  component, not embedded in the titlebar). Populate from recent reading
  bookmarks (§3.3 option 1).
- D2 Polish existing `dt-window-chrome`: hover/active feedback on the
  `— ❐ ✕` buttons; ensure breadcrumb stays readable on all four themes;
  verify drag region works once the tab strip is added (drag on empty
  tab-strip space, not on a tab button).
- D3 **No traffic-light dots.** Skip that part of the kit deliberately —
  noted here so the next person doesn't "fix" it.
- D4 Optional: real "open books" model behind a feature flag (§3.3
  option 2).

### Phase E · Admin package (UI only, mocked data)

- E0 Workspace member + Cargo manifest + entry binaries (web + desktop).
- E1 Window chrome (same OS-agnostic `dt-window-chrome` as the reader app
  — breadcrumb says `Musanif Admin › <section>`) plus the shared
  `dt-tabstrip` for `Admin · Library · Analytics` top-level tabs.
- E2 Admin sidebar (queue + library + system + user).
- E3 Book header + StageStepper.
- E4 Stage panels in order: Upload → Publish (one PR each).
- E5 Wire the right-side panels (provider picker / cost / metadata) to the
  same `ui::state` patterns as the reader app.

### Phase F · Admin backend wiring (separate effort)

Out of scope for this plan; see §6.

---

## 5. Component extraction checklist (what *must* land in `ui`)

These are the exact components the kits use across more than one platform.
Anything not on this list stays in its consuming package.

- [ ] `Island` (variants: default, soft, quiet)
- [ ] `Cover` ✅ — extend with `accent` gradient
- [ ] `NavIsland` (Browse · Library · Collections groups, user footer)
- [ ] `RailIsland`
- [ ] `ContinueRail` + `ContinueCard` + `HighlightCard` + `ReadingGoalCard`
- [ ] `FeatureCard`
- [ ] `PageHeader` (title · subtitle · actions slot)
- [ ] `StatTile` (used by profile + admin)
- [ ] `TabStrip` — Slack-style content tabs (Shelf, Settings sub-nav, admin chapter list)
- [ ] `WindowTabStrip` — OS-agnostic chrome tab row used by desktop reader (open books) and admin (top-level sections); rendered *below* the existing `dt-window-chrome`, never replacing it
- [ ] `KbdHint`
- [ ] `Chip` (already in CSS as `.is-chip`; expose as RSX component)
- [ ] `Button` (`.is-btn`, `--primary`, `--ghost`, `--danger`, `--block`)
- [ ] `IconButton` (`.is-icon-btn`)
- [ ] `ProgressBar` (`.is-progress`)
- [ ] Settings: `Field`, `SegOption`, `Toggle` ✅ (already in
      `views/settings/toggle.rs` — promote to `ui::components::toggle`)

After extraction, each existing view should drop substantial inline JSX in
favor of these components. Net-LOC reduction is a good signal it worked.

---

## 6. Out-of-scope (deliberately)

- **Real ingestion pipeline** (PDF parsing, Urdu OCR, LLM calls, embeddings,
  job queue, blob storage). The admin UI is built against mock data per the
  kit; we wire endpoints in a follow-up.
- **Multi-window / multi-tab reader state machine.** The chapter-tab strip
  starts as a recent-books proxy.
- **Analytics page in admin.** Tab is shown but the route is a placeholder
  ("Coming soon").
- **i18n / RTL framework.** RTL Urdu rendering is per-element via `dir="rtl"
  lang="ur"`, as the kits do; no global RTL inversion yet.

---

## 7. Open decisions for review

1. **Admin as separate crate** (recommended) vs feature-flagged route inside
   `ui`. Plan above assumes separate crate.
2. **Chapter-tabs source**: recent-bookmarks proxy first, real "open books"
   later? (See §3.3.)
3. **Settings Library section** scope — sync settings, downloads, cache
   clear? Or just placeholder for now?
4. **Phase order** — does B (reader-app polish) need to ship before E
   (admin), or run in parallel? The admin work doesn't *need* B done;
   primitives from A unblock both.

---

## 8. Quick metric for "done"

When a phase is complete, the matching kit screen and the running app should
be visually indistinguishable at the component level (same classes, same
layout, same theme reactivity). Theme switching parchment ↔ midnight should
have no visual regressions.
