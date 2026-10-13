use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::OutboxMessage;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

#[async_trait]
pub trait OutboxEventPort: Send + Sync {
    async fn claim(
        &self,
        owner: Uuid,
        batch: u32,
        now: NaiveDateTime,
        lease_until: NaiveDateTime,
    ) -> Result<Vec<OutboxMessage>, NucleoException>;
    async fn complete(
        &self,
        message: &OutboxMessage,
        owner: Uuid,
        now: NaiveDateTime,
    ) -> Result<bool, NucleoException>;
    async fn fail(
        &self,
        message: &OutboxMessage,
        owner: Uuid,
        next_retry_at: NaiveDateTime,
    ) -> Result<bool, NucleoException>;
    async fn purge(
        &self,
        outbox_cutoff: NaiveDateTime,
        processed_cutoff: NaiveDateTime,
    ) -> Result<u64, NucleoException>;
}
