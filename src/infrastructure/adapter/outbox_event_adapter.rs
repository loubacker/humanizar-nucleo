use crate::domain::exception::NucleoException;
use crate::domain::model::messaging::OutboxMessage;
use crate::domain::port::messaging::OutboxEventPort;
use crate::infrastructure::persistence::repository::OutboxEventRepository;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

pub(crate) struct OutboxEventAdapter {
    repository: OutboxEventRepository,
}

impl OutboxEventAdapter {
    pub fn new(repository: OutboxEventRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl OutboxEventPort for OutboxEventAdapter {
    async fn claim(
        &self,
        owner: Uuid,
        batch: u32,
        now: NaiveDateTime,
        lease_until: NaiveDateTime,
    ) -> Result<Vec<OutboxMessage>, NucleoException> {
        self.repository
            .claim(owner, batch, now, lease_until)
            .await?
            .into_iter()
            .map(to_message)
            .collect()
    }

    async fn complete(
        &self,
        message: &OutboxMessage,
        owner: Uuid,
        now: NaiveDateTime,
    ) -> Result<bool, NucleoException> {
        self.repository.complete(message, owner, now).await
    }

    async fn fail(
        &self,
        message: &OutboxMessage,
        owner: Uuid,
        next_retry_at: NaiveDateTime,
    ) -> Result<bool, NucleoException> {
        self.repository.fail(message, owner, next_retry_at).await
    }

    async fn purge(
        &self,
        outbox_cutoff: NaiveDateTime,
        processed_cutoff: NaiveDateTime,
    ) -> Result<u64, NucleoException> {
        self.repository.purge(outbox_cutoff, processed_cutoff).await
    }
}

fn to_message(
    entity: crate::infrastructure::persistence::entity::messaging::OutboxEventEntity,
) -> Result<OutboxMessage, NucleoException> {
    let reason = crate::domain::model::enums::ReasonCode::DatabaseAccessFailed;
    let attempts = u32::try_from(entity.attempt_count)
        .map_err(|error| NucleoException::with_source(reason, error))?;
    let maximum = u32::try_from(entity.max_attempts)
        .map_err(|error| NucleoException::with_source(reason, error))?;
    if maximum == 0 {
        return Err(NucleoException::new(reason));
    }
    Ok(OutboxMessage::new(
        entity.id.ok_or_else(|| NucleoException::new(reason))?,
        entity.event_id,
        entity.correlation_id,
        entity.exchange_name,
        entity.routing_key,
        entity.payload,
        attempts,
        maximum,
    ))
}
