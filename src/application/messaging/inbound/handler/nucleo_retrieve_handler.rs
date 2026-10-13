use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::{NucleoRetrieveCommand, ProcessedEvent, ProcessingOutcome};
use crate::domain::port::messaging::ProcessedEventPort;
use std::sync::Arc;

pub struct NucleoRetrieveHandler {
    processed: Arc<dyn ProcessedEventPort>,
}

impl NucleoRetrieveHandler {
    pub fn new(processed: Arc<dyn ProcessedEventPort>) -> Self {
        Self { processed }
    }

    pub async fn handle(
        &self,
        command: NucleoRetrieveCommand,
    ) -> Result<ProcessingOutcome, NucleoException> {
        self.processed.process(command).await
    }

    pub async fn record_failure(
        &self,
        event: ProcessedEvent,
    ) -> Result<ProcessingOutcome, NucleoException> {
        self.processed.record_failure(event).await
    }
}
