//! GraphQL client, powered by cynic codegen against
//! `musanif-contracts/schema.graphql`.
//!
//! Each submodule defines:
//! - cynic-derived response/input types matching the schema's camelCase
//!   wire format (snake_case Rust idents + cynic's auto-rename),
//! - thin wrapper functions that take/return the existing `models::*`
//!   types so views are insulated from cynic.
//!
//! The whole layer is funnelled through `client::run`, which adds the
//! bearer-auth header and transparently retries once on `Unauthorized`
//! after exchanging the refresh token.

mod client;
mod schema;

mod admin;
mod auth;
mod books;
mod highlights;
mod me;
mod mutations;
mod subscriptions;

pub use client::GqlError;

// ── Catalogue ────────────────────────────────────────────────────────────────
pub use books::{
    fetch_author, fetch_authors, fetch_book, fetch_book_reviews, fetch_books,
    fetch_books_by_author, fetch_chapter, fetch_chapters, fetch_featured,
    fetch_word_translations,
};

// ── Auth & 2FA ───────────────────────────────────────────────────────────────
pub use auth::{
    disable_2fa, forgot_password, login, login_2fa_complete, logout_user, refresh_token,
    register, request_email_verification, reset_password_with_token, setup_2fa, verify_2fa,
    verify_email,
};

// ── Authenticated user ───────────────────────────────────────────────────────
pub use me::{
    change_password, delete_me, fetch_me, fetch_my_2fa_status, fetch_my_bookmarks,
    fetch_my_following, fetch_my_highlights, fetch_my_reading_goal, fetch_my_reading_sessions,
    fetch_my_stats, update_profile, upsert_reading_goal,
};

// ── Mutations with input objects ─────────────────────────────────────────────
pub use mutations::{record_reading_session, submit_book_review, upsert_bookmark};

// ── Reader annotations ───────────────────────────────────────────────────────
pub use highlights::{create_highlight, fetch_chapter_highlights};

// ── Admin / pipeline ─────────────────────────────────────────────────────────
pub use admin::{
    approve_chapter_draft, create_ingestion_job, fetch_admin_job, fetch_admin_jobs,
    fetch_admin_usage, fetch_chapter_drafts, fetch_cover_variants, fetch_job_log,
    fetch_job_steps, fetch_publish_checks, flag_chapter_draft, publish_ingestion_job,
    select_cover_variant, start_ingestion_job, ChapterDraft, CoverVariant, IngestionJob,
    JobEventSub, JobLogEntry, PipelineStep, PublishCheck, PublishedBook, UsageOverview,
};

// ── Subscriptions ────────────────────────────────────────────────────────────
pub use subscriptions::{subscribe_job_events, JobEventCallback};
