# musanif

Dioxus 0.7 frontend for the Musanif Urdu reading platform — one Rust workspace, five platform shells.

```
musanif/
├── packages/
│   ├── ui/         # shared library: components, views, routes, GraphQL client
│   ├── web/        # browser entry point (WASM)
│   ├── desktop/    # tao/wry shell with custom title bar
│   ├── mobile/     # iOS + Android shell
│   └── admin/      # admin console (separate binary, drives the Sarab pipeline)
└── docs/           # design north-stars (read these before changing the API surface)
```

Every shell is intentionally thin: it sets up the launcher, applies the persisted theme, mounts `ui::Route`, and contributes any platform-specific chrome. Everything reusable lives in `packages/ui`.

## Backend

Talks to [`merk`](../merk) over GraphQL at `/api/graphql`. The transport is generated from [`musanif-contracts/schema.graphql`](../musanif-contracts/schema.graphql) via [cynic 3.13](https://github.com/obmarg/cynic) — `packages/ui/build.rs` registers the schema for cynic codegen on every compile, so type names in `#[derive(QueryFragment)]` resolve directly.

When the backend's GraphQL surface changes, refresh the contract:

```bash
cargo run --manifest-path ../merk/Cargo.toml --bin merk-dump-schema
# → ../musanif-contracts/schema.graphql
```

## Run

```bash
cd packages/web        && dx serve
cd packages/desktop    && dx serve --platform desktop
cd packages/mobile     && dx serve --platform ios   # or android
cd packages/admin      && dx serve --platform desktop
```

Each shell reads `GRAPHQL_URL` and `API_BASE_URL` from `musanif/.env.local` at compile time (defaults to `http://localhost:9678/api/graphql` and `http://localhost:9678/api/v1`). Override per environment by setting `BUILD_PROFILE` (e.g. `BUILD_PROFILE=prod` loads `musanif/.env.prod`).

## Token storage

`packages/ui/src/api/token.rs` exposes `set_auth_token` / `set_refresh_token` / `clear_all_tokens`. Web persists both via `localStorage`; desktop and mobile fall back to in-memory storage today (native secure-storage adapters — `keyring`, Keychain, EncryptedSharedPreferences — are tracked in `INTEGRATION_PLAN.md` Phase 3).

## Theming + preferences

- `ui::theme::apply_and_persist(theme)` writes a `data-theme` attribute on `<html>` and persists the choice to `localStorage`.
- `ui::prefs::use_persisted_{string,bool,u32}` round-trips display/reading/library settings through `localStorage`. The shape is key/value so it can be swapped for a server-side `update_preferences` mutation when one lands.

## Layout (per package)

See each package's README for the per-shell breakdown:
- [`packages/ui`](packages/ui/README.md) — shared layer (where almost all the code lives)
- [`packages/web`](packages/web/README.md)
- [`packages/desktop`](packages/desktop/README.md)
- [`packages/mobile`](packages/mobile/README.md)
- [`packages/admin`](packages/admin/README.md)

## Tooling

```bash
cargo check                                          # whole workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
cargo run --manifest-path ../merk/Cargo.toml --bin merk-dump-schema
```

The Dioxus CLI (`dx`) is required for `dx serve` and `dx build`:

```bash
cargo install dioxus-cli
```

## Docs

- [`docs/`](docs/) — design north-stars (component kit, ui-implementation-plan, api-integration-plan)
- [`../INTEGRATION_PLAN.md`](../INTEGRATION_PLAN.md) — phased plan for the merk ↔ musanif cutover

## License

CC0-1.0
