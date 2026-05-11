# desktop

Desktop shell for the Musanif reader, built on Dioxus's tao/wry backend. Adds a frameless window with a custom title bar — drag region, breadcrumb, and platform-style window controls — over `ui::Route`.

## Layout

```
src/
└── main.rs   # window config (no decorations, custom title bar) + App component
assets/
└── main.css
```

The decorationless `WindowBuilder` and the `WindowTab` / `WindowTabStrip` components live here because they're desktop-specific chrome. The router and views come from [`ui`](../ui).

## Run

```bash
cd musanif/packages/desktop
dx serve --platform desktop
```

The desktop build talks to the same backend the web build does (`GRAPHQL_URL`, `API_BASE_URL` env vars compiled in via `packages/ui/build.rs`). On first run, configure those in `musanif/.env.local` to point at your merk instance.

## Token storage caveat

`api::token` falls back to in-memory storage on desktop today, so the user is signed out on every restart. A `keyring`-backed `TokenStore` impl is queued in `INTEGRATION_PLAN.md` Phase 3.

## License

CC0-1.0
