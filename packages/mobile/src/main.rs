//! iOS + Android entry point for the Musanif reader.
//!
//! Mounts [`ui::Route`]; the `mobile` feature on the `ui` crate flips
//! routes to their `views::mobile::*` shells where the mobile-specific
//! layout differs from the desktop/web one.

use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::logger::initialize_default();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<ui::Route> {}
    }
}
