//! Shared UI library: models, API client, routes, and views.

pub mod api;
pub mod models;
pub mod views;

mod navbar;
mod routes;

pub use navbar::{AppNavbar, Navbar, NAVIGATE_HOME};
pub use routes::Route;
