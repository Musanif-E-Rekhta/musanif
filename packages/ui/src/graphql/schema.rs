//! Cynic-generated GraphQL schema bindings.
//!
//! `build.rs` registers `musanif-contracts/schema.graphql` as the default
//! schema for cynic codegen, so the macro below loads it without a path.
//! All `#[derive(cynic::QueryFragment)]` structs in sibling modules
//! resolve their type names against this module.

#[cynic::schema("musanif")]
pub mod schema {}

// The schema's `DateTime` custom scalar is just RFC-3339 over the wire.
// Map it to `chrono::DateTime<Utc>` so callers can pass the same chrono
// values they already use in `chapter_reader.rs` etc., and so cynic's
// derives accept the field types directly.
cynic::impl_scalar!(chrono::DateTime<chrono::Utc>, schema::DateTime);
