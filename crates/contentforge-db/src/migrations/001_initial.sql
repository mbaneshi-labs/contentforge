-- Content: the central entity.
CREATE TABLE content (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    content_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'idea',
    tags TEXT NOT NULL DEFAULT '[]',  -- JSON array
    project TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Platform-specific adaptations of content.
CREATE TABLE adaptations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_id TEXT NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    title TEXT,
    body TEXT NOT NULL,
    thread_parts TEXT,  -- JSON array, nullable
    canonical_url TEXT,
    metadata TEXT NOT NULL DEFAULT '{}',  -- JSON object
    UNIQUE(content_id, platform)
);

-- Media attachments.
CREATE TABLE media (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    alt_text TEXT
);

-- Platform account credentials.
CREATE TABLE platform_accounts (
    id TEXT PRIMARY KEY,
    platform TEXT NOT NULL,
    display_name TEXT NOT NULL,
    credential TEXT NOT NULL,  -- JSON (encrypted in production)
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Publication records (what was published where).
CREATE TABLE publications (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    url TEXT NOT NULL,
    platform_post_id TEXT NOT NULL,
    published_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Schedule entries.
CREATE TABLE schedule (
    id TEXT PRIMARY KEY,
    content_id TEXT NOT NULL REFERENCES content(id) ON DELETE CASCADE,
    platform TEXT NOT NULL,
    scheduled_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    error TEXT,
    retries INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Recurring schedules.
CREATE TABLE recurring_schedules (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cron_expr TEXT NOT NULL,
    template TEXT,
    platforms TEXT NOT NULL DEFAULT '[]',  -- JSON array
    enabled INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Analytics snapshots.
CREATE TABLE analytics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    publication_id TEXT NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
    views INTEGER DEFAULT 0,
    likes INTEGER DEFAULT 0,
    shares INTEGER DEFAULT 0,
    comments INTEGER DEFAULT 0,
    clicks INTEGER DEFAULT 0,
    captured_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Indexes.
CREATE INDEX idx_content_status ON content(status);
CREATE INDEX idx_content_project ON content(project);
CREATE INDEX idx_adaptations_content ON adaptations(content_id);
CREATE INDEX idx_publications_content ON publications(content_id);
CREATE INDEX idx_schedule_status ON schedule(status);
CREATE INDEX idx_schedule_time ON schedule(scheduled_at);
CREATE INDEX idx_analytics_publication ON analytics(publication_id);
