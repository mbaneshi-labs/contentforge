use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use contentforge_core::{ContentStatus, Platform};
use contentforge_db::repo::{ContentRepo, PublicationRepo};
use contentforge_db::DbPool;
use contentforge_pipeline::JobQueue;
use contentforge_publish::PublisherRegistry;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub tick_interval: Duration,
    pub max_retries: u32,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            tick_interval: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

// ---------------------------------------------------------------------------
// Scheduler — handles both one-off schedule entries and cron recurring
// ---------------------------------------------------------------------------

pub struct Scheduler {
    pub config: SchedulerConfig,
    pub db: DbPool,
    pub publishers: Arc<PublisherRegistry>,
}

impl Scheduler {
    pub fn new(db: DbPool, publishers: Arc<PublisherRegistry>, config: SchedulerConfig) -> Self {
        Self {
            config,
            db,
            publishers,
        }
    }

    /// Run the scheduler loop. Checks for due entries and cron triggers.
    pub async fn run(&self) -> Result<()> {
        tracing::info!(
            interval_secs = self.config.tick_interval.as_secs(),
            "Scheduler started"
        );

        loop {
            tokio::time::sleep(self.config.tick_interval).await;

            // Process one-off scheduled entries
            if let Err(e) = self.process_scheduled_entries().await {
                tracing::error!(error = %e, "Scheduled entries tick failed");
            }

            // Process cron recurring schedules
            if let Err(e) = self.process_cron_schedules().await {
                tracing::error!(error = %e, "Cron schedules tick failed");
            }
        }
    }

    // -----------------------------------------------------------------------
    // One-off schedule entries (from `schedule` table)
    // -----------------------------------------------------------------------

    async fn process_scheduled_entries(&self) -> Result<()> {
        let entries = self.get_pending_entries()?;
        let now = Utc::now();

        for entry in entries {
            if entry.scheduled_at <= now {
                tracing::info!(
                    entry_id = %entry.id,
                    platform = %entry.platform_str,
                    "Publishing scheduled entry"
                );
                self.process_entry(&entry).await;
            }
        }
        Ok(())
    }

    fn get_pending_entries(&self) -> Result<Vec<ScheduleEntryRow>> {
        let conn = self.db.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, content_id, platform, scheduled_at, status, retries
             FROM schedule
             WHERE status = 'pending' AND scheduled_at <= datetime('now')
             ORDER BY scheduled_at ASC
             LIMIT 20",
        )?;

        let rows = stmt
            .query_map([], |row| {
                let scheduled_str = row.get::<_, String>(3)?;
                let scheduled_at = chrono::DateTime::parse_from_rfc3339(&scheduled_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now());
                Ok(ScheduleEntryRow {
                    id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                    content_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                    platform_str: row.get::<_, String>(2)?,
                    scheduled_at,
                    status: row.get::<_, String>(4)?,
                    retries: row.get::<_, i32>(5)? as u32,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    async fn process_entry(&self, entry: &ScheduleEntryRow) {
        self.update_status(entry.id, "in_progress");

        match self.try_publish(entry).await {
            Ok(url) => {
                tracing::info!(entry_id = %entry.id, url = %url, "Published");
                self.update_status(entry.id, "published");
            }
            Err(e) => {
                tracing::error!(entry_id = %entry.id, error = %e, "Publish failed");
                if entry.retries < self.config.max_retries {
                    self.increment_retry(entry.id);
                } else {
                    tracing::error!(entry_id = %entry.id, "Max retries exceeded");
                    self.update_status_with_error(entry.id, "failed", &e.to_string());
                }
            }
        }
    }

    async fn try_publish(&self, entry: &ScheduleEntryRow) -> Result<String> {
        let platform: Platform = serde_json::from_str(&entry.platform_str).or_else(|_| {
            entry
                .platform_str
                .trim_matches('"')
                .parse::<Platform>()
                .map_err(|e| anyhow::anyhow!(e))
        })?;

        let content_repo = ContentRepo::new(self.db.clone());
        let content = content_repo
            .get_by_id_full(entry.content_id)?
            .ok_or_else(|| anyhow::anyhow!("Content not found: {}", entry.content_id))?;

        let adaptation = content
            .adaptation_for(platform)
            .ok_or_else(|| anyhow::anyhow!("No adaptation for {platform}"))?;

        let publisher = self
            .publishers
            .get(platform)
            .ok_or_else(|| anyhow::anyhow!("No publisher configured for {platform}"))?;

        let publication = publisher
            .publish(&content, adaptation)
            .await
            .map_err(|e| anyhow::anyhow!("Publish failed: {e}"))?;

        // Save publication record
        let pub_repo = PublicationRepo::new(self.db.clone());
        pub_repo.insert(&publication)?;

        // Update content status
        content_repo.update_status(entry.content_id, ContentStatus::Published)?;

        Ok(publication.url)
    }

    fn update_status(&self, id: Uuid, status: &str) {
        if let Ok(conn) = self.db.lock() {
            let _ = conn.execute(
                "UPDATE schedule SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
                rusqlite::params![status, id.to_string()],
            );
        }
    }

    fn update_status_with_error(&self, id: Uuid, status: &str, error: &str) {
        if let Ok(conn) = self.db.lock() {
            let _ = conn.execute(
                "UPDATE schedule SET status = ?1, error = ?2, updated_at = datetime('now') WHERE id = ?3",
                rusqlite::params![status, error, id.to_string()],
            );
        }
    }

    fn increment_retry(&self, id: Uuid) {
        if let Ok(conn) = self.db.lock() {
            let _ = conn.execute(
                "UPDATE schedule SET retries = retries + 1, status = 'pending', updated_at = datetime('now') WHERE id = ?1",
                [id.to_string()],
            );
        }
    }

    // -----------------------------------------------------------------------
    // Cron recurring schedules (from `recurring_schedules` table)
    // -----------------------------------------------------------------------

    async fn process_cron_schedules(&self) -> Result<()> {
        let schedules = self.get_enabled_cron_schedules()?;

        for schedule in schedules {
            if self.cron_is_due(&schedule.cron_expr) {
                tracing::info!(
                    name = %schedule.name,
                    cron = %schedule.cron_expr,
                    "Cron schedule triggered"
                );
                self.trigger_cron_pipeline(&schedule);
            }
        }
        Ok(())
    }

    fn get_enabled_cron_schedules(&self) -> Result<Vec<CronScheduleRow>> {
        let conn = self.db.lock().map_err(|e| anyhow::anyhow!("{e}"))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, cron_expr, template, platforms, enabled
             FROM recurring_schedules WHERE enabled = 1",
        )?;

        let rows = stmt
            .query_map([], |row| {
                Ok(CronScheduleRow {
                    id: row.get::<_, String>(0)?,
                    name: row.get::<_, String>(1)?,
                    cron_expr: row.get::<_, String>(2)?,
                    template: row.get::<_, Option<String>>(3)?,
                    platforms_json: row.get::<_, String>(4)?,
                })
            })?
            .collect::<std::result::Result<Vec<_>, _>>()?;

        Ok(rows)
    }

    fn cron_is_due(&self, cron_expr: &str) -> bool {
        let schedule = match cron::Schedule::from_str(cron_expr) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(cron = cron_expr, error = %e, "Invalid cron expression");
                return false;
            }
        };

        let now = Utc::now();
        let tick = self.config.tick_interval;

        // Check if there's a cron occurrence within the tick window
        if let Some(next) = schedule.upcoming(Utc).next() {
            let diff = (next - now).num_seconds().unsigned_abs();
            diff <= tick.as_secs()
        } else {
            false
        }
    }

    fn trigger_cron_pipeline(&self, schedule: &CronScheduleRow) {
        let queue = JobQueue::new(self.db.clone());
        let pipeline = schedule.template.as_deref().unwrap_or("publish-all");

        let platforms: Vec<String> = serde_json::from_str(&schedule.platforms_json)
            .unwrap_or_else(|_| vec!["devto".to_string()]);

        let payload = serde_json::json!({
            "platforms": platforms,
            "skip_review": false,
            "triggered_by": "cron",
            "cron_schedule": schedule.name,
        });

        match queue.enqueue(
            pipeline, "adapt",
            None, // cron pipelines don't have a specific content_id — they create one
            None, None, payload, None,
        ) {
            Ok(job) => {
                tracing::info!(
                    job_id = %job.id,
                    schedule = %schedule.name,
                    "Cron pipeline job enqueued"
                );
            }
            Err(e) => {
                tracing::error!(
                    schedule = %schedule.name,
                    error = %e,
                    "Failed to enqueue cron pipeline"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Internal row types
// ---------------------------------------------------------------------------

struct ScheduleEntryRow {
    id: Uuid,
    content_id: Uuid,
    platform_str: String,
    #[allow(dead_code)]
    scheduled_at: chrono::DateTime<Utc>,
    #[allow(dead_code)]
    status: String,
    retries: u32,
}

struct CronScheduleRow {
    #[allow(dead_code)]
    id: String,
    name: String,
    cron_expr: String,
    template: Option<String>,
    platforms_json: String,
}
