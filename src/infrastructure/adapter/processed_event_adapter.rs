use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::{NucleoRetrieveCommand, ProcessedEvent, ProcessingOutcome};
use crate::domain::port::messaging::ProcessedEventPort;
use crate::infrastructure::persistence::repository::ProcessedEventRepository;
use async_trait::async_trait;

pub(crate) struct ProcessedEventAdapter {
    repository: ProcessedEventRepository,
}

impl ProcessedEventAdapter {
    pub fn new(repository: ProcessedEventRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl ProcessedEventPort for ProcessedEventAdapter {
    async fn process(
        &self,
        command: NucleoRetrieveCommand,
    ) -> Result<ProcessingOutcome, NucleoException> {
        self.repository.process(command).await
    }

    async fn record_failure(
        &self,
        event: ProcessedEvent,
    ) -> Result<ProcessingOutcome, NucleoException> {
        self.repository.record_failure(event).await
    }
}
