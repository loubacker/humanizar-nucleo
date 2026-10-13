use crate::application::catalog::{AggregateCatalog, ProducerCatalog, RoutingKeyCatalog};
use crate::application::messaging::outbound::dto::OutboundEnvelopeDto;
use crate::application::messaging::outbound::mapper::NucleoCallbackMapper;
use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use crate::domain::model::messaging::{NucleoRetrieveCommand, OutboxEvent};
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

pub struct NucleoCallbackFactory;
impl NucleoCallbackFactory {
    pub fn create(
        command: &NucleoRetrieveCommand,
        nome: Option<String>,
        now: NaiveDateTime,
        max_attempts: u32,
    ) -> Result<OutboxEvent, NucleoException> {
        match nome {
            Some(nome) => Self::encode(
                command,
                RoutingKeyCatalog::NUCLEO_RETRIEVED_V1,
                NucleoCallbackMapper::retrieved(command, nome),
                now,
                max_attempts,
            ),
            None => Self::encode(
                command,
                RoutingKeyCatalog::NUCLEO_REJECTED_V1,
                NucleoCallbackMapper::rejected(command),
                now,
                max_attempts,
            ),
        }
    }

    fn encode<T: Serialize>(
        command: &NucleoRetrieveCommand,
        routing_key: &str,
        payload: T,
        now: NaiveDateTime,
        max_attempts: u32,
    ) -> Result<OutboxEvent, NucleoException> {
        let event_id = Uuid::new_v4();
        let envelope = OutboundEnvelopeDto {
            event_id,
            correlation_id: command.correlation_id,
            producer_service: ProducerCatalog::NUCLEO_SERVICE.to_owned(),
            exchange_name: command.callback_exchange.clone(),
            routing_key: routing_key.to_owned(),
            aggregate_type: AggregateCatalog::NUCLEO.to_owned(),
            aggregate_id: command.nucleo_id,
            event_version: 1,
            occurred_at: now,
            actor_id: command.actor_id,
            user_agent: command.user_agent.clone(),
            origin_ip: command.origin_ip.clone(),
            payload,
        };
        let serialized = serde_json::to_string(&envelope)
            .map_err(|error| NucleoException::with_source(ReasonCode::UnexpectedError, error))?;
        Ok(OutboxEvent::create(
            event_id,
            command.correlation_id,
            ProducerCatalog::NUCLEO_SERVICE,
            command.callback_exchange.clone(),
            routing_key,
            AggregateCatalog::NUCLEO,
            command.nucleo_id,
            1,
            serialized,
            command.actor_id,
            command.user_agent.clone(),
            command.origin_ip.clone(),
            max_attempts,
            now,
        ))
    }
}
