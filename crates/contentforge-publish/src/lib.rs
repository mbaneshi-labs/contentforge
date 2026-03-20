pub mod adapters;

use async_trait::async_trait;
use contentforge_core::{Content, ContentForgeError, Platform, PlatformAdaptation, Publication};

/// The core trait every platform adapter must implement.
#[async_trait]
pub trait Publisher: Send + Sync {
    /// Which platform this adapter handles.
    fn platform(&self) -> Platform;

    /// Validate that the content meets platform constraints before publishing.
    fn validate(&self, adaptation: &PlatformAdaptation) -> Result<(), ContentForgeError> {
        if let Some(limit) = self.platform().char_limit() {
            if adaptation.body.len() > limit {
                return Err(ContentForgeError::ContentTooLong {
                    platform: self.platform(),
                    limit,
                    actual: adaptation.body.len(),
                });
            }
        }
        Ok(())
    }

    /// Publish the adapted content to the platform.
    async fn publish(
        &self,
        content: &Content,
        adaptation: &PlatformAdaptation,
    ) -> Result<Publication, ContentForgeError>;

    /// Delete a previously published piece of content.
    async fn delete(&self, publication: &Publication) -> Result<(), ContentForgeError>;

    /// Check if the adapter is properly authenticated.
    async fn health_check(&self) -> Result<(), ContentForgeError>;
}

/// Registry of all configured platform adapters.
pub struct PublisherRegistry {
    publishers: Vec<Box<dyn Publisher>>,
}

impl PublisherRegistry {
    pub fn new() -> Self {
        Self {
            publishers: Vec::new(),
        }
    }

    pub fn register(&mut self, publisher: Box<dyn Publisher>) {
        self.publishers.push(publisher);
    }

    pub fn get(&self, platform: Platform) -> Option<&dyn Publisher> {
        self.publishers
            .iter()
            .find(|p| p.platform() == platform)
            .map(|p| p.as_ref())
    }

    /// Publish content to all platforms it has adaptations for.
    pub async fn publish_all(
        &self,
        content: &Content,
    ) -> Vec<Result<Publication, ContentForgeError>> {
        let mut results = Vec::new();
        for adaptation in &content.adaptations {
            if let Some(publisher) = self.get(adaptation.platform) {
                let result = publisher.publish(content, adaptation).await;
                results.push(result);
            } else {
                results.push(Err(ContentForgeError::PlatformNotConfigured(
                    adaptation.platform,
                )));
            }
        }
        results
    }
}

impl Default for PublisherRegistry {
    fn default() -> Self {
        Self::new()
    }
}
