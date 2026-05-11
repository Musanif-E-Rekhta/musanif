# mobile

Mobile shell (iOS + Android) for the Musanif reader. Mounts `ui::Route` and turns on the `mobile` feature on `ui` so the reader, profile, and discover views render with mobile-only chrome from `ui::views::mobile`.

## Layout

```
src/
└── main.rs   # `App` component: stylesheet + Router::<ui::Route>
assets/
└── main.css
```

This crate is intentionally thin — every screen is built in [`ui`](../ui), with platform-specific layouts living in `ui::views::mobile::*`. Branching happens at the top of each route component (`if cfg!(feature = "mobile") { … }`).

## Run

```bash
cd musanif/packages/mobile

# iOS simulator
dx serve --platform ios

# Android emulator
dx serve --platform android
```

You'll need the corresponding mobile toolchain (`cargo-ndk` for Android, Xcode for iOS).

## Token storage caveat

`api::token` falls back to in-memory storage on mobile, so the user is signed out on every cold start. Keychain (iOS) and EncryptedSharedPreferences (Android) `TokenStore` impls are queued in `INTEGRATION_PLAN.md` Phase 3.

## License

CC0-1.0
