//! Public client surface for the Musanif backend.
//!
//! Per INTEGRATION_PLAN.md §4 the transport is GraphQL by default. The
//! domain operations (`fetch_books`, `login`, `submit_book_review`, …)
//! are defined in [`crate::graphql`] and re-exported here so the views
//! continue to use the historical `api::*` namespace.
//!
//! Two REST endpoints survive multipart upload and binary asset fetch
//! and live in the [`uploads`] and [`binary`] submodules.

pub(crate) mod http;
mod token;

pub mod binary;
pub mod uploads;

// Authoritative GraphQL operations re-exported here so existing call
// sites (`api::fetch_books`, `api::login`, …) keep resolving while the
// transport is GraphQL underneath.
pub use crate::graphql::*;

pub use token::{
    clear_all_tokens, get_auth_token, get_refresh_token, set_auth_token, set_refresh_token,
};
