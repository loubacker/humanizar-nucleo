use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::{NucleoRetrieveCommand, ProcessedEvent, ProcessingOutcome};
use async_trait::async_trait;

#[async_trait]
pub trait ProcessedEventPort: Send + Sync {
    async fn process(
        &self,
        command: NucleoRetrieveCommand,
    ) -> Result<ProcessingOutcome, NucleoException>;

    async fn record_failure(
        &self,
        event: ProcessedEvent,
    ) -> Result<ProcessingOutcome, NucleoException>;
}
