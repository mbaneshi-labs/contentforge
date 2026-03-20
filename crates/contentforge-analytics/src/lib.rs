use anyhow::Result;
use chrono::{DateTime, Utc};
use contentforge_core::Platform;
use contentforge_db::DbPool;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Analytics types
// ---------------------------------------------------------------------------

/// Engagement metrics for a single publication.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub views: u64,
    pub likes: u64,
    pub comments: u64,
    pub shares: u64,
    pub clicks: u64,
}

/// Analytics for a single published content item on one platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicationAnalytics {
    pub content_id: uuid::Uuid,
    pub platform: Platform,
    pub published_at: DateTime<Utc>,
    pub metrics: EngagementMetrics,
    pub last_fetched: DateTime<Utc>,
}

/// Summary of all content in the pipeline.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineSummary {
    pub total_ideas: u64,
    pub total_drafts: u64,
    pub total_in_review: u64,
    pub total_ready: u64,
    pub total_scheduled: u64,
    pub total_published: u64,
    pub total_archived: u64,
}

/// Per-platform aggregated stats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformStats {
    pub platform: Platform,
    pub total_published: u64,
    pub total_views: u64,
    pub total_likes: u64,
    pub total_comments: u64,
    pub avg_engagement_rate: f64,
}

/// Top-level dashboard analytics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardAnalytics {
    pub pipeline: PipelineSummary,
    pub platform_stats: Vec<PlatformStats>,
    pub recent_publications: Vec<PublicationAnalytics>,
}

// ---------------------------------------------------------------------------
// Aggregation functions (stubs)
// ---------------------------------------------------------------------------

/// Get the pipeline summary (content counts by status).
pub fn pipeline_summary(_db: &DbPool) -> Result<PipelineSummary> {
    // TODO: Query DB for counts per ContentStatus
    todo!("Aggregate pipeline summary from DB")
}

/// Get per-platform analytics.
pub fn platform_stats(_db: &DbPool) -> Result<Vec<PlatformStats>> {
    // TODO: Query DB for per-platform aggregation
    todo!("Aggregate platform stats from DB")
}

/// Get the full dashboard analytics.
pub fn dashboard(_db: &DbPool) -> Result<DashboardAnalytics> {
    // TODO: Combine pipeline_summary + platform_stats + recent publications
    todo!("Build full dashboard analytics")
}

/// Record engagement metrics for a publication (called by platform adapters).
pub fn record_metrics(
    _db: &DbPool,
    _content_id: uuid::Uuid,
    _platform: Platform,
    _metrics: EngagementMetrics,
) -> Result<()> {
    // TODO: Upsert metrics into analytics table
    todo!("Record engagement metrics")
}
