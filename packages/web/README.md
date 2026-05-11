# web

The browser shell for the Musanif reader. Wraps `ui::Route` in a Dioxus router and serves at `dx serve`'s default port. WebAssembly target.

## Layout

```
src/
└── main.rs   # `App` component: applies the persisted theme and mounts ui::Route
assets/
├── favicon.ico
└── main.css
```

`web` adds nothing on top of [`ui`](../ui) other than the entry point and the WASM-target dependencies (e.g. `web-sys` is pulled by `ui` only under `cfg(target_arch = "wasm32")`). Anything reusable across platforms should land in `ui`.

## Run

```bash
# from repo root
cd musanif/packages/web
dx serve                         # default platform = web
```

The dev server expects the merk backend on `http://localhost:9678/api/graphql` (override with `GRAPHQL_URL` in `musanif/.env.local` — see `packages/ui/build.rs`).

## Build for production

```bash
dx build --release --platform web
# output: dist/
```

To embed the artefact into the merk single-binary deploy, build with `EMBED_FRONTEND=true` per the merk Dockerfile.

## License

CC0-1.0
