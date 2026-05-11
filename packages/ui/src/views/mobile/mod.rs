//! Mobile-specific view implementations. Each existing reader-app view
//! delegates here when the `mobile` feature flag is on; otherwise it
//! renders the desktop variant.

pub mod authors;
pub mod discover;
pub mod library;
pub mod profile;
pub mod reader;
pub mod settings;
