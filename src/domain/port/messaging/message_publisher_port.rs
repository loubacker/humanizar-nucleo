use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::OutboxMessage;
use async_trait::async_trait;

#[async_trait]
pub trait MessagePublisherPort: Send + Sync {
    fn is_available(&self) -> bool;

    async fn publish(&self, message: &OutboxMessage) -> Result<(), NucleoException>;
}
