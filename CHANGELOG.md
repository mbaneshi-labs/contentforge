# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace structure with 11 modular crates
- Core domain types: `Content`, `ContentStatus`, `ContentType`, `Platform`, `PlatformAdaptation`, `Publication`
- Platform credential model: `PlatformCredential` (API key, OAuth2, integration token, cookie)
- Schedule types: `ScheduleEntry`, `ScheduleStatus`, `RecurringSchedule`
- Error handling with `ContentForgeError` (thiserror) and `anyhow`
- SQLite database layer with WAL mode and migration framework
- Database schema: content, adaptations, media, platform_accounts, publications, schedule, recurring_schedules, analytics
- Content repository (`ContentRepo`) with insert, get, list, update status, delete
- `Publisher` trait with validate, publish, delete, health_check methods
- `PublisherRegistry` for managing and dispatching to multiple platform adapters
- DEV.to adapter: create articles with tags, series, and canonical URL support
- Twitter/X adapter: single tweets and threaded replies with rate limit detection
- LinkedIn adapter: posts via REST API with proper versioning headers
- Medium adapter: articles via integration token with Markdown content format
- Platform metadata: character limits, markdown support, image support, thread support, integration difficulty ratings

[Unreleased]: https://github.com/mbaneshi/contentforge/compare/v0.1.0...HEAD
