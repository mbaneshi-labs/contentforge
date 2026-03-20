use crate::platform::Platform;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A scheduled publication event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub id: Uuid,
    pub content_id: Uuid,
    pub platform: Platform,
    pub scheduled_at: DateTime<Utc>,
    pub status: ScheduleStatus,
    /// If failed, the error message.
    pub error: Option<String>,
    /// Number of retry attempts.
    pub retries: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleStatus {
    Pending,
    InProgress,
    Published,
    Failed,
    Cancelled,
}

/// A recurring content schedule (e.g., "weekly ship post every Friday").
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringSchedule {
    pub id: Uuid,
    pub name: String,
    /// Cron expression (e.g., "0 8 * * FRI" = every Friday at 8 AM).
    pub cron_expr: String,
    /// Template to use for auto-generation.
    pub template: Option<String>,
    pub platforms: Vec<Platform>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
