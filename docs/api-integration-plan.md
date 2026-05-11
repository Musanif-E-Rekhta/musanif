# Musanif Client — Backend API Integration Plan

> Companion to `../../merk/docs/api-and-db-plan.md` (the backend plan)
> and `ui-implementation-plan.md` (the UI plan in this repo). This
> document tracks **client-side** work to consume the merk backend's
> GraphQL-first transport policy.
>
> **Status:** plan only. No client-code changes yet — review and adjust
> before starting any of this.

---

## 1. Transport policy (mirror of merk §1)

merk has resolved the "REST or GraphQL?" question. Going forward, on
the client:

- **GraphQL is the default.** All authenticated reads/writes, anything
  composing multiple entities, the entire admin surface, and live
  updates go through `packages/ui/src/graphql/*`.
- **REST stays — frozen — for three explicit reasons:**
  1. Public, cacheable `GET`s (book list/detail, author list/detail,
     taxonomy, search, featured).
  2. File upload (admin upload + pre-signed finalize).
  3. Binary asset rendering (PDF page previews).
- **Don't extend existing REST helpers** in `packages/ui/src/api/` other
  than the three categories above. Bug fixes only.
- **No new "both transports" features.** Pick one per feature, per the
  per-section tags in the merk plan's §2 / §3.

This is the *client* mirror — the *backend* policy is the source of
truth. If those documents drift, merk wins.

---

## 2. Current client surface (summary)

`packages/ui/src/api/` (REST):
- `auth.rs`, `me.rs`, `authors.rs`, `books.rs`, `bookmarks.rs`,
  `chapters.rs`, `collections.rs`, `comments.rs`, `goals.rs`,
  `highlights.rs`, `reviews.rs`, `taxonomy.rs`, `translations.rs`
- Shared: `http.rs` (request helper), `token.rs` (auth-token storage)

`packages/ui/src/graphql/` (GraphQL):
- `auth.rs`, `me.rs`, `books.rs`, `mutations.rs`
- Shared: `client.rs`, `macros.rs` (`gql_op!`)

Many domains exist in *both* trees today. No domain is GraphQL-only;
several (`comments`, `taxonomy`, etc.) are REST-only.

---

## 3. Per-domain disposition

| Domain         | REST today                       | GraphQL today              | Direction                              | Notes |
| -------------- | -------------------------------- | -------------------------- | -------------------------------------- | ----- |
| **Auth**       | `api/auth.rs`                    | `graphql/auth.rs`          | **GraphQL only** (incl. new 2FA)       | Migrate `register`/`login`/`logout`/`forgot`/`reset` to GQL; delete `api/auth.rs` once views are off it |
| **Me / profile** | `api/me.rs`                    | `graphql/me.rs`            | **GraphQL only**                       | New fields (`plan`, `pace_hint`, expanded `UserStats`) only land in GQL — REST helper stays at current shape, then deleted |
| **Books**      | `api/books.rs`                   | `graphql/books.rs`         | **REST primary** for list + detail     | Public, cacheable. New code calls `api::books::*`. GQL `books`/`book` queries stay for now but are not extended |
| **Authors**    | `api/authors.rs`                 | partial (`graphql/books.rs`) | **REST primary** for list + detail; **GraphQL** for `is_following` | Use REST for the list/detail page; switch to GQL for the `/authors/:slug` page when `is_following` matters |
| **Taxonomy**   | `api/taxonomy.rs`                | none                       | **REST**                               | Public + cacheable. No change |
| **Chapters**   | `api/chapters.rs`                | partial (`graphql/books.rs`) | **REST primary**, GQL OK for chapter detail (already there) | Reader page uses GQL `chapter(book_slug, chapter_slug)` for prev/next; list view stays REST |
| **Bookmarks**  | `api/bookmarks.rs`               | `mutations.rs::UpsertBookmark`, `me.rs::MyShelf` | **GraphQL only**            | Continue rail and `order` arg are GQL-only; REST helper deleted after Phase 2 |
| **Reviews**    | `api/reviews.rs`                 | `books.rs::BookReviews`    | **GraphQL only**                       | Mutating + relational |
| **Comments**   | `api/comments.rs`                | none                       | **GraphQL only** (new)                 | Build `graphql/comments.rs` from scratch; deprecate `api/comments.rs` once views switch |
| **Highlights** | `api/highlights.rs`              | `me.rs::MyHighlights`      | **GraphQL only**                       | The new `order` arg lands only in GQL; per-chapter highlights move to GQL |
| **Translations** | `api/translations.rs`          | `books.rs::WordTranslations` | **GraphQL only**                     | Vote + submit mutations migrate to GQL |
| **Collections** | `api/collections.rs`            | none                       | **GraphQL only** (new)                 | Build `graphql/collections.rs`; deprecate `api/collections.rs` |
| **Goals**      | `api/goals.rs`                   | `me.rs::ReadingGoal`, `mutations.rs::SetGoal` | **GraphQL only**     | `pace_hint` / `on_track` only on GQL type |
| **Sessions**   | none                             | none                       | **GraphQL only** (new)                 | `RecordReadingSession` mutation |
| **Featured**   | none                             | none                       | **REST** (new)                         | `api/featured.rs`; cacheable |
| **Search**     | none                             | none                       | **REST** (new, optional)               | `api/search.rs`; ⌘K UX uses existing `fetch_books`/`fetch_authors` until then |
| **Admin · uploads** | none                        | none                       | **REST** (new)                         | Multipart + pre-signed finalize |
| **Admin · page preview** | none                   | none                       | **REST** (new)                         | Returns binary `.png` |
| **Admin · everything else** | none                | none                       | **GraphQL only** (new)                 | Jobs, drafts, AI/usage, covers, publish, library, analytics, subscriptions |

---

## 4. New client modules to create

### GraphQL (`packages/ui/src/graphql/`)

- `me_continue.rs` — `query MyContinue` (§2.2 backend)
- `me_plan.rs` — extends `Me { plan }` selection (§2.11)
- `me_totp.rs` — 2FA setup/verify/disable + status (§2.12)
- `me_prefs.rs` — reading preferences (§2.6, **deferred**)
- `sessions.rs` — `RecordReadingSession` mutation (§2.7)
- `comments.rs` — full comment CRUD + voting (currently REST only)
- `collections.rs` — full collection CRUD (currently REST only)
- `subscriptions.rs` — `JobEvents`, `DraftEvents` (§3.3, §3.4) + a
  `stream_<thing>` helper that returns a `Stream`
- `admin/jobs.rs` — list/get + start/pause/resume/cancel + config update
- `admin/drafts.rs` — list/get/update/approve/flag/reject + re-OCR
- `admin/ai.rs` — providers, models, usage, budget
- `admin/covers.rs` — generate + select
- `admin/publish.rs` — publish-checks + publish
- `admin/library.rs` — admin books query, update visibility, unpublish
- `admin/authors.rs` — match + create + update

### REST (`packages/ui/src/api/`)

- `featured.rs` — `GET /api/v1/featured` (§2.1)
- `search.rs` — `GET /api/v1/search` (§2.8, optional)
- `admin/uploads.rs` — direct multipart + pre-signed `sign` + `finalize`
- `admin/page_preview.rs` — fetch `…/pages/{n}.png` as bytes

### Shared plumbing

- `graphql/client.rs` — add WebSocket transport for subscriptions
  (`graphql-ws` over `tokio-tungstenite`).
- `graphql/macros.rs` — add a sibling to `gql_op!` for subscriptions
  that returns `impl Stream<Item = Result<T, Error>>` instead of
  `Future<Output = Result<T, Error>>`.
- `state.rs` — admin app currents (`CURRENT_JOB`, `JOB_FILTER`) + a
  shared subscription handle pool so duplicate subs are deduped.

---

## 5. Path-prefix cleanup

`packages/ui/src/api/*.rs` mixes prefixes:

- Some functions hit bare paths (`/auth/register`, `/me/bookmarks`,
  `/books/{slug}`).
- Others hit prefixed paths (`/api/v1/auth/forgot-password`,
  `/api/v1/me`, `/api/v1/me/profile`).

The backend mounts everything under `/api/v1/*` consistently. Whichever
form the client uses must be uniform, otherwise `api_base_url()`'s value
is ambiguous and one variant is silently broken depending on its
configuration.

**Action:**
- Set `config::api_base_url()` to **not** include `/api/v1`.
- Update every REST helper to prefix its path with `/api/v1/...`
  literally (or pass through a single `rest_path("auth/login")` helper
  that prepends).
- Add a clippy lint or `grep` check in CI: `grep -R '"\/auth\|"\/me\|"\/books\|"\/authors\|"\/categories\|"\/tags\|"\/chapters\|"\/comments\|"\/translations\|"\/highlights\|"\/reviews\|"\/collections\|"\/translations'` should return zero hits in `packages/ui/src/api/`.

This is a one-shot cleanup; ships in Phase 1 below.

---

## 6. Subscription transport

Today there is no subscription support in the client.
`graphql/client.rs` only does single-shot HTTP POST queries. To support
the merk admin pipeline's `JobEvents` / `DraftEvents`:

1. **Add a WS transport** beside the HTTP one. Use the
   `graphql-transport-ws` sub-protocol (`async-graphql`'s default).
   Crate: `tokio-tungstenite` for the WS itself; small handshake state
   machine on top. Auth token rides in the `connection_init` payload.
2. **Keep one connection per app session.** Demultiplex multiple
   subscriptions over the same WS — don't open one per subscription.
3. **Macro split.** `gql_op!` builds a `Future<Output = Result<T>>`.
   For subscriptions add `gql_sub!` that builds an
   `impl Stream<Item = Result<T>>`. Both compile-check the operation
   document at build time the same way `gql_op!` does today.
4. **Reconnect with backoff.** If the WS drops, re-open and re-issue
   active subscription operations. The Process / Review screens
   should not freeze during a transient network blip.
5. **Fallback contract.** If WS handshake fails entirely, the helper
   degrades to polling the corresponding query (`JobSteps`, `JobLog`)
   on a 2s interval. This is the only fallback — no SSE, no REST.

---

## 7. Phasing

Aligned with merk's `B-back` / `B2-back` / `F1-back` / `F3-back` so
client + server PRs land together.

### Phase 1 · Foundation (no UI change)

- [ ] Path-prefix cleanup (§5).
- [ ] Resolve duplicate `me` call sites (REST `/me` vs GQL `me { ... }`).
  Pick GQL going forward; leave REST helper but stop calling it from
  views.
- [ ] Add `gql_sub!` macro and WS transport scaffold (§6) — even though
  no subscription op exists yet on the server. Land it dark, gated by a
  feature flag if needed.

### Phase 2 · Reader-app gaps (pairs with merk B-back)

- [ ] `graphql/me_continue.rs` + wire `ContinueRail` to it
  (`views/home.rs`, `views/profile.rs`).
- [ ] Extend `MyStats` GQL selection with `hours_read`, `day_streak`;
  wire `views/profile.rs` stat tiles.
- [ ] Extend `ReadingGoal` GQL selection with `pace_hint`, `on_track`;
  pass into `reading_goal_card.rs`.
- [ ] Extend `Me` GQL selection with `plan { tier, name, features }`;
  wire `views/settings/account.rs`.
- [ ] `graphql/sessions.rs` + heartbeat call from reader on unmount /
  every 60s.
- [ ] Add `is_following` to the `Author` GQL selection used by
  `views/authors/detail.rs`; wire the Follow / Following toggle.
- [ ] Add `order` arg to `MyHighlights` and `MyShelf` queries; use
  `created_desc` in profile recent-highlights and `completed_at_desc`
  in profile recently-finished.
- [ ] `api/featured.rs` + wire `feature_card.rs` on Discover.
- [ ] *Optional:* `api/search.rs` + ⌘K palette behavior. If skipped,
  the ⌘K hint is purely visual and routes to existing search.
- [ ] *Deferred:* `graphql/me_prefs.rs` (only if UI plan B7 needs
  cross-device persistence).

### Phase 2.5 · 2FA (pairs with merk B2-back)

- [ ] `graphql/me_totp.rs`.
- [ ] Wire the toggle in `views/settings/account.rs` to call
  `Setup2fa` → show QR + recovery codes → `Verify2fa`.
- [ ] Update `views/auth/login.rs`: handle the
  `LoginUser` → `Login2faChallenge` branch and chain into
  `Login2faComplete`.

### Phase 3 · Migrate existing REST domains to GraphQL

This is the bulk of the cleanup. Per-domain PRs:

- [ ] Comments — new `graphql/comments.rs`, swap callers in reader +
  highlight views, delete `api/comments.rs`.
- [ ] Collections — same pattern.
- [ ] Reviews — switch all callers from `api/reviews.rs` to GQL; delete
  the REST module.
- [ ] Highlights — same.
- [ ] Translations — same.
- [ ] Bookmarks — same. Continue rail already reads from GQL by then.
- [ ] Authors / Books / Chapters — **only follow** state and chapter
  detail move; list endpoints stay REST.

Each PR is small (one domain) and ends with the REST file deleted.

### Phase 4 · Admin client (pairs with merk E + F1)

- [ ] `packages/admin/` workspace member; same shell as desktop reader.
- [ ] `api/admin/uploads.rs` (drop-zone direct or pre-signed).
- [ ] `api/admin/page_preview.rs` (`<img>` for PDF pages).
- [ ] `graphql/admin/jobs.rs` + wire Process screen.
- [ ] `graphql/admin/drafts.rs` + wire Review/Edit screens.
- [ ] `graphql/admin/ai.rs` + wire provider picker + cost card.
- [ ] `graphql/admin/covers.rs` + wire cover-variant gallery.
- [ ] `graphql/admin/publish.rs` + wire pre-flight checks + Publish button.
- [ ] `graphql/admin/library.rs` + wire Library tab.
- [ ] `graphql/admin/authors.rs` + wire metadata-resolution UI.

Until F3-back lands, the admin app polls `JobSteps` / `JobLog` /
`ChapterDrafts` on a 2s timer (the §6 fallback path). No code change
when subscriptions go live: the helper detects WS support and switches.

### Phase 5 · Subscriptions (pairs with merk F3-back)

- [ ] Replace polling helpers with `JobEvents` / `DraftEvents`
  subscription calls. Same view code, different transport.
- [ ] Add reconnect-with-backoff once a real backend exists to test
  against.
- [ ] Verify the 2s polling fallback still works for environments
  without WS.

---

## 8. Deprecation policy for existing REST helpers

We don't delete in a flag day. The order is always:

1. **Land the GraphQL replacement** in a new `graphql/<domain>.rs`.
2. **Migrate one view at a time** to the GQL helper. Each view PR is
   independently revertable.
3. **Once every call site is migrated**, delete the REST module in a
   final cleanup PR. Don't leave dead helpers — they invite drift.

Domains where the REST helper stays *forever*: `api/books.rs`,
`api/authors.rs` (list endpoints), `api/taxonomy.rs`,
`api/chapters.rs` (list endpoint), `api/featured.rs`, `api/search.rs`,
`api/admin/uploads.rs`, `api/admin/page_preview.rs`. Don't extend them
beyond the merk-plan-defined shape.

---

## 9. Open decisions

1. **WS library.** `tokio-tungstenite` (mature, no Dioxus integration
   needed) vs. browser-native `WebSocket` via `web-sys` for the web
   target. Likely two paths gated by `cfg(target_arch = "wasm32")`.
2. **Subscription multiplexing.** One WS per app session is the
   recommendation; reconsider if the admin app ends up with >20
   simultaneous subscriptions per user (probably won't).
3. **GraphQL code generation.** Today we hand-write GQL ops in `.rs`
   files. As the surface doubles for admin, consider `cynic` or
   `graphql_client` for compile-time-checked types from a
   `schema.graphql` snapshot. Defer until pain becomes real.
4. **Auth token in subscriptions.** Send via the `connection_init`
   payload (recommended) vs. as a query param (simpler but logs leak
   it). Going with `connection_init`.
5. **Phase 3 ordering.** Migrate noisy / low-risk domains first
   (translations, collections) before high-traffic ones (reviews,
   bookmarks)? Probably yes — confirms the migration template before
   touching hot paths.

---

## 10. Done criteria

| Phase     | Definition of done                                                                                              |
| --------- | --------------------------------------------------------------------------------------------------------------- |
| Phase 1   | One canonical path scheme in `api/*.rs`. WS scaffold compiles and connects (no subs in use yet).                |
| Phase 2   | Discover/Profile/Reader screens render real data: Continue rail, expanded stats, pace hint, plan tier, sessions writing back. |
| Phase 2.5 | 2FA flow round-trips: toggle on, scan, verify, log out, log back in with code.                                  |
| Phase 3   | Every domain in §3's "GraphQL only" rows has its REST module deleted; views call only GQL.                      |
| Phase 4   | `packages/admin/` runs against the real merk backend (no mock data). All five admin stages function end-to-end. |
| Phase 5   | Admin Process screen log + step strip update live without polling. No code change in views vs. Phase 4.         |
