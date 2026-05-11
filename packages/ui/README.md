# ui

Shared Dioxus components, views, routing, and the GraphQL client. Every platform shell (`web`, `desktop`, `mobile`, `admin`) depends on this crate — they only contribute the entry point, asset paths, and any platform-specific chrome.

## Layout

```
src/
├── api/         # public surface used by views (re-exports graphql::*)
├── components/  # design-system pieces (Cover, FeatureCard, AuthField, …)
├── graphql/     # cynic-typed operations + thin wrappers to models::*
├── models.rs    # domain types consumed by views
├── views/       # one file per route (Home, BookDetail, ChapterReader, …)
├── prefs.rs     # localStorage-backed preference helpers
├── theme.rs     # theme application + persistence
├── routes.rs    # Route enum (driven by dioxus-router)
├── state.rs     # global signals (CURRENT_USER, CURRENT_THEME, …)
├── navbar.rs    # navigation chrome shared across platforms
├── mobile_shell.rs # mobile-only outer shell
└── lib.rs       # re-exports the crate's public surface
```

## GraphQL client

The `graphql/` module is generated from [`musanif-contracts/schema.graphql`](../../../musanif-contracts/schema.graphql) via [cynic 3.13](https://github.com/obmarg/cynic). `build.rs` registers the schema as the default for cynic's derive macros, so `#[derive(cynic::QueryFragment)]` resolves type names automatically.

Adding a new query/mutation:

```rust
// in graphql/<domain>.rs
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "MyVars")]
pub struct MyQuery {
    #[arguments(slug: $slug)]
    pub thing: Option<ThingGql>,
}

pub async fn fetch_thing(slug: String) -> Option<models::Thing> {
    let op = MyQuery::build(MyVars { slug });
    super::client::run(op).await.and_then(|d| d.thing.map(Into::into))
}
```

`client::run` adds the bearer-auth header and transparently retries once on `Unauthorized` after exchanging the refresh token.

## Token storage

`api::token` provides `set_auth_token` / `set_refresh_token` / `clear_all_tokens`. On web, both tokens persist via `localStorage`; on desktop and mobile, they currently fall back to in-memory storage (native secure-storage adapters are tracked in `INTEGRATION_PLAN.md` Phase 3 follow-ups).

## Features

| Feature | Effect |
|---------|--------|
| `mobile` | Branches `Home`/`ChapterReader`/`Profile` into `views/mobile/*` shells |

## Platform constraint

Don't add platform-specific dependencies (e.g. `web-sys` outside a `target_arch = "wasm32"` cfg, or desktop-only bindings) to this crate. Put them in the corresponding [`web`](../web), [`desktop`](../desktop), [`mobile`](../mobile), or [`admin`](../admin) crate so the shared layer stays portable.

## License

CC0-1.0
