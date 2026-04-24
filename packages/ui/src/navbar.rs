use dioxus::prelude::*;

use crate::Route;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

/// Set to `true` from outside the router (e.g. a custom desktop titlebar)
/// to trigger a home navigation from inside the router context.
pub static NAVIGATE_HOME: GlobalSignal<bool> = Signal::global(|| false);

/// Generic slot-based navbar used by legacy platform wrappers.
#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        div { id: "navbar", {children} }
    }
}

/// Router layout that wraps every page with the shared navigation bar.
#[component]
pub fn AppNavbar() -> Element {
    let nav = use_navigator();

    use_effect(move || {
        if *NAVIGATE_HOME.read() {
            *NAVIGATE_HOME.write() = false;
            nav.push(Route::Home {});
        }
    });

    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }
        nav { id: "navbar",
            div { class: "navbar-links",
                Link { to: Route::Home {}, "Discover" }
                Link { to: Route::Shelf {}, "My Shelf" }
            }
            div { class: "navbar-brand",
                Link { to: Route::Home {}, "مصنف" }
            }
        }
        Outlet::<Route> {}
    }
}
