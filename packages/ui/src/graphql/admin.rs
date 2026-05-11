//! Admin GraphQL operations — ingestion queue, drafts, AI usage,
//! cover variants, publish gates.
//!
//! Subscriptions (`onPipelineStep`, `onPipelineLog`, `onPipelineCompleted`)
//! aren't here — they need a graphql-transport-ws client which is a
//! separate concern. The admin UI polls these queries today; subscription
//! wire-up is tracked as a Phase 4 follow-up.

use chrono::{DateTime, Utc};
use cynic::{MutationBuilder, QueryBuilder};

use super::client::run;
use super::schema::schema;
use crate::models;

// ── Cynic types ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct IngestionJobGql {
    pub id: String,
    pub asset_id: String,
    pub book_id: Option<String>,
    pub hint_title: Option<String>,
    pub hint_title_ur: Option<String>,
    pub hint_author: Option<String>,
    pub pages: Option<i32>,
    pub size_bytes: Option<i32>,
    pub stage: i32,
    pub status: String,
    pub ai_provider: Option<String>,
    pub ai_model: Option<String>,
    pub overall_confidence: Option<f64>,
    pub chapters_total: Option<i32>,
    pub chapters_flagged: Option<i32>,
    pub tokens_used: i32,
    pub est_cost_usd: f64,
    pub cover_color: Option<String>,
    pub cover_glyph: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct PipelineStepGql {
    pub id: String,
    pub n: i32,
    pub label: String,
    pub status: String,
    pub detail: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct JobLogEntryGql {
    pub id: String,
    pub t: Option<DateTime<Utc>>,
    pub kind: String,
    pub message: String,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct ChapterDraftGql {
    pub id: String,
    pub job_id: String,
    pub n: i32,
    pub title_ur: String,
    pub title_en: Option<String>,
    pub page_range: String,
    pub ai_content: String,
    pub ai_content_format: String,
    pub human_content: Option<String>,
    pub ai_summary: Option<String>,
    pub themes: Vec<String>,
    pub entities: Vec<String>,
    pub confidence: f64,
    pub status: String,
    pub flag_reason: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct UsageOverviewGql {
    pub period: String,
    pub tokens_used: i32,
    pub est_cost_usd: f64,
    pub monthly_budget_usd: f64,
    pub budget_used_pct: f64,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct CoverVariantGql {
    pub id: String,
    pub job_id: String,
    pub bucket: String,
    pub object: String,
    pub model_id: Option<String>,
    pub prompt: Option<String>,
    pub is_selected: bool,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct PublishCheckGql {
    pub ok: bool,
    pub gate: Option<String>,
    pub label: String,
    pub detail: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
pub struct PublishedBookGql {
    pub id: String,
    pub slug: String,
}

// ── Domain types re-exposed for view callers ─────────────────────────────────

pub type IngestionJob = IngestionJobGql;
pub type PipelineStep = PipelineStepGql;
pub type JobLogEntry = JobLogEntryGql;
pub type ChapterDraft = ChapterDraftGql;
pub type UsageOverview = UsageOverviewGql;
pub type CoverVariant = CoverVariantGql;
pub type PublishCheck = PublishCheckGql;
pub type PublishedBook = PublishedBookGql;

// ── Variables / Inputs ────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables, Debug)]
pub struct AdminJobsVars {
    pub status: Option<String>,
    pub stage: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct IdVars {
    pub id: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct JobVars {
    pub job: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct JobLogVars {
    pub job: String,
    pub since: Option<String>,
    pub limit: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct AdminUsageVars {
    pub period: Option<String>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct CreateIngestionJobVars {
    pub input: CreateIngestionJobInputCynic,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct FlagDraftVars {
    pub id: String,
    pub reason: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SelectCoverVars {
    pub job: String,
    pub variant: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct PublishVars {
    pub job: String,
    pub input: PublishInputCynic,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "CreateIngestionJobInput")]
pub struct CreateIngestionJobInputCynic {
    pub asset_id: String,
    pub hint_title: Option<String>,
    pub hint_author: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "PublishInput")]
pub struct PublishInputCynic {
    pub visibility: String,
    pub schedule_at: Option<DateTime<Utc>>,
}

// ── Queries ───────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "AdminJobsVars")]
pub struct AdminJobsQuery {
    #[arguments(status: $status, stage: $stage, limit: $limit, offset: $offset)]
    pub ingestion_jobs: Vec<IngestionJobGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "IdVars")]
pub struct AdminJobQuery {
    #[arguments(id: $id)]
    pub ingestion_job: Option<IngestionJobGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "JobVars")]
pub struct JobStepsQuery {
    #[arguments(job: $job)]
    pub job_steps: Vec<PipelineStepGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "JobLogVars")]
pub struct JobLogQuery {
    #[arguments(job: $job, since: $since, limit: $limit)]
    pub job_log: Vec<JobLogEntryGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "JobVars")]
pub struct ChapterDraftsQuery {
    #[arguments(job: $job)]
    pub chapter_drafts: Vec<ChapterDraftGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "AdminUsageVars")]
pub struct AdminUsageQuery {
    #[arguments(period: $period)]
    pub admin_usage: UsageOverviewGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "JobVars")]
pub struct CoverVariantsQuery {
    #[arguments(job: $job)]
    pub cover_variants: Vec<CoverVariantGql>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "QueryRoot", variables = "JobVars")]
pub struct PublishChecksQuery {
    #[arguments(job: $job)]
    pub publish_checks: Vec<PublishCheckGql>,
}

// ── Mutations ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "CreateIngestionJobVars")]
pub struct CreateIngestionJobMutation {
    #[arguments(input: $input)]
    pub create_ingestion_job: IngestionJobGql,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "IdVars")]
pub struct StartIngestionJobMutation {
    #[arguments(id: $id)]
    pub start_ingestion_job: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "IdVars")]
pub struct ApproveDraftMutation {
    #[arguments(id: $id)]
    pub approve_chapter_draft: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "FlagDraftVars")]
pub struct FlagDraftMutation {
    #[arguments(id: $id, reason: $reason)]
    pub flag_chapter_draft: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "SelectCoverVars")]
pub struct SelectCoverMutation {
    #[arguments(job: $job, variant: $variant)]
    pub select_cover_variant: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "MutationRoot", variables = "PublishVars")]
pub struct PublishMutation {
    #[arguments(job: $job, input: $input)]
    pub publish_ingestion_job: PublishedBookGql,
}

// ── Public API ────────────────────────────────────────────────────────────────

pub async fn fetch_admin_jobs(
    status: Option<String>,
    stage: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Option<Vec<IngestionJob>> {
    let op = AdminJobsQuery::build(AdminJobsVars {
        status,
        stage,
        limit,
        offset,
    });
    run(op).await.map(|d| d.ingestion_jobs)
}

pub async fn fetch_admin_job(id: String) -> Option<IngestionJob> {
    let op = AdminJobQuery::build(IdVars { id });
    run(op).await.and_then(|d| d.ingestion_job)
}

pub async fn fetch_job_steps(job: String) -> Option<Vec<PipelineStep>> {
    let op = JobStepsQuery::build(JobVars { job });
    run(op).await.map(|d| d.job_steps)
}

pub async fn fetch_job_log(
    job: String,
    since: Option<String>,
    limit: Option<i32>,
) -> Option<Vec<JobLogEntry>> {
    let op = JobLogQuery::build(JobLogVars { job, since, limit });
    run(op).await.map(|d| d.job_log)
}

pub async fn fetch_chapter_drafts(job: String) -> Option<Vec<ChapterDraft>> {
    let op = ChapterDraftsQuery::build(JobVars { job });
    run(op).await.map(|d| d.chapter_drafts)
}

pub async fn fetch_admin_usage(period: Option<String>) -> Option<UsageOverview> {
    let op = AdminUsageQuery::build(AdminUsageVars { period });
    run(op).await.map(|d| d.admin_usage)
}

pub async fn fetch_cover_variants(job: String) -> Option<Vec<CoverVariant>> {
    let op = CoverVariantsQuery::build(JobVars { job });
    run(op).await.map(|d| d.cover_variants)
}

pub async fn fetch_publish_checks(job: String) -> Option<Vec<PublishCheck>> {
    let op = PublishChecksQuery::build(JobVars { job });
    run(op).await.map(|d| d.publish_checks)
}

pub async fn create_ingestion_job(
    asset_id: String,
    hint_title: Option<String>,
    hint_author: Option<String>,
) -> Option<IngestionJob> {
    let op = CreateIngestionJobMutation::build(CreateIngestionJobVars {
        input: CreateIngestionJobInputCynic {
            asset_id,
            hint_title,
            hint_author,
        },
    });
    run(op).await.map(|d| d.create_ingestion_job)
}

pub async fn start_ingestion_job(id: String) -> bool {
    let op = StartIngestionJobMutation::build(IdVars { id });
    run(op).await.map(|d| d.start_ingestion_job).unwrap_or(false)
}

pub async fn approve_chapter_draft(id: String) -> bool {
    let op = ApproveDraftMutation::build(IdVars { id });
    run(op)
        .await
        .map(|d| d.approve_chapter_draft)
        .unwrap_or(false)
}

pub async fn flag_chapter_draft(id: String, reason: String) -> bool {
    let op = FlagDraftMutation::build(FlagDraftVars { id, reason });
    run(op).await.map(|d| d.flag_chapter_draft).unwrap_or(false)
}

pub async fn select_cover_variant(job: String, variant: String) -> bool {
    let op = SelectCoverMutation::build(SelectCoverVars { job, variant });
    run(op).await.map(|d| d.select_cover_variant).unwrap_or(false)
}

pub async fn publish_ingestion_job(
    job: String,
    visibility: String,
) -> Option<PublishedBook> {
    let op = PublishMutation::build(PublishVars {
        job,
        input: PublishInputCynic {
            visibility,
            schedule_at: None,
        },
    });
    run(op).await.map(|d| d.publish_ingestion_job)
}

// ── Subscriptions ────────────────────────────────────────────────────────────
//
// `JobEventGql` is a single object discriminated by `kind`:
//   - "step_update"          → uses n/label/status/detail/started_at/finished_at
//   - "log_entry"            → uses t/log_kind/message
//   - "chapter_draft_added"  → uses n/draft_id
//   - "pipeline_completed"   → uses status
// The frontend dispatches on `kind` to fold each event into the right signal.

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
#[cynic(graphql_type = "JobEventGql")]
pub struct JobEventSub {
    pub kind: String,
    pub job_id: String,
    pub n: Option<i32>,
    pub label: Option<String>,
    pub status: Option<String>,
    pub detail: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub t: Option<DateTime<Utc>>,
    pub log_kind: Option<String>,
    pub message: Option<String>,
    pub draft_id: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, PartialEq)]
#[cynic(graphql_type = "DraftEventGql")]
pub struct DraftEventSub {
    pub kind: String,
    pub job_id: String,
    pub draft_id: String,
    pub by_user: Option<String>,
    pub reason: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "SubscriptionRoot", variables = "JobVars")]
pub struct JobEventsSubscription {
    #[arguments(job: $job)]
    pub job_events: JobEventSub,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "SubscriptionRoot", variables = "JobVars")]
pub struct DraftEventsSubscription {
    #[arguments(job: $job)]
    pub draft_events: DraftEventSub,
}

// Suppress unused warnings — these types are part of the admin API
// surface even when the admin UI hasn't migrated all consumers yet.
#[allow(dead_code)]
fn _force_use() -> Option<(IngestionJob, PipelineStep, JobLogEntry, ChapterDraft, UsageOverview, CoverVariant, PublishCheck, models::User, JobEventSub, DraftEventSub)> {
    None
}
