# admin

The Musanif admin console — a separate Dioxus binary that drives the Sarab ingestion pipeline (upload, process, review, edit, publish). Re-uses [`ui`](../ui)'s components, theme, GraphQL client, and routing infrastructure.

## Layout

```
src/
├── main.rs       # App component: sidebar + per-stage view selector
├── lib.rs        # re-exports for components/state/views
├── components/   # admin-specific chrome (Sidebar, BookHeader, StageStepper, …)
├── state.rs     # AdminSection / CURRENT_JOB / CURRENT_STAGE signals
└── views/       # one view per pipeline stage:
    ├── upload.rs
    ├── process.rs
    ├── review.rs
    ├── edit.rs
    └── publish.rs
```

## Backend wiring

Every admin operation goes through `ui::api::*` (which re-exports the cynic-typed admin operations from `ui::graphql::admin`). Queries: `fetch_admin_jobs`, `fetch_admin_job`, `fetch_job_steps`, `fetch_job_log`, `fetch_chapter_drafts`, `fetch_admin_usage`, `fetch_cover_variants`, `fetch_publish_checks`. Mutations: `create_ingestion_job`, `start_ingestion_job`, `approve_chapter_draft`, `flag_chapter_draft`, `select_cover_variant`, `publish_ingestion_job`.

Live pipeline updates are still polled — the GraphQL subscriptions transport (`graphql-transport-ws`) is a Phase 4 follow-up tracked in `INTEGRATION_PLAN.md`.

## Run

```bash
cd musanif/packages/admin

# Default — desktop window via Dioxus desktop runtime
dx serve --platform desktop

# Or as a regular WASM SPA
dx serve --platform web
```

## Features

| Feature | Effect |
|---------|--------|
| `web` (default) | Targets `wasm32` via Dioxus's web runtime |
| `desktop` | Targets the desktop runtime (tao/wry window) |

## License

CC0-1.0
