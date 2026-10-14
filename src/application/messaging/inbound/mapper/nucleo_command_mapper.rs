use crate::application::catalog::{ExchangeCatalog, RoutingKeyCatalog};
use crate::application::messaging::inbound::dto::{InboundEnvelopeDto, NucleoRetrieveCommandDto};
use crate::application::validator::required;
use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use crate::domain::model::messaging::NucleoRetrieveCommand;

pub struct NucleoCommandMapper;
impl NucleoCommandMapper {
    pub fn failed_event(
        body: &[u8],
        reason: ReasonCode,
        now: chrono::NaiveDateTime,
    ) -> Option<crate::domain::model::messaging::ProcessedEvent> {
        if body.len() > 2 * 1024 * 1024 {
            return None;
        }
        let envelope: InboundEnvelopeDto<serde_json::Value> = serde_json::from_slice(body).ok()?;
        let aggregate_type = envelope
            .aggregate_type
            .filter(|value| !value.trim().is_empty() && value.chars().count() <= 255)?;
        let routing_key = envelope
            .routing_key
            .filter(|value| value.chars().count() <= 255)?;
        if envelope
            .user_agent
            .as_ref()
            .is_some_and(|value| value.chars().count() > 255)
            || envelope
                .origin_ip
                .as_ref()
                .is_some_and(|value| value.chars().count() > 255)
        {
            return None;
        }
        Some(crate::domain::model::messaging::ProcessedEvent::new(
            crate::application::catalog::ConsumerCatalog::RETRIEVE_CONSUMER,
            envelope.event_id?,
            None,
            None,
            envelope.correlation_id?,
            routing_key,
            aggregate_type,
            envelope.aggregate_id?,
            envelope.actor_id,
            envelope.user_agent,
            envelope.origin_ip,
            now,
            crate::domain::model::enums::messaging::ProcessedResult::Failed,
            Some(reason),
            Some(reason.message().to_owned()),
        ))
    }

    pub fn parse(
        body: &[u8],
        exchange: &str,
        routing_key: &str,
    ) -> Result<NucleoRetrieveCommand, NucleoException> {
        if body.len() > 2 * 1024 * 1024 {
            return Err(NucleoException::new(ReasonCode::ValidationError));
        }
        let envelope: InboundEnvelopeDto<NucleoRetrieveCommandDto> = serde_json::from_slice(body)
            .map_err(|error| {
            NucleoException::with_source(ReasonCode::InboundParseError, error)
        })?;
        let missing = || NucleoException::new(ReasonCode::InboundRequiredField);
        let event_id = envelope.event_id.ok_or_else(missing)?;
        let correlation_id = envelope.correlation_id.ok_or_else(missing)?;
        required(envelope.producer_service, ReasonCode::InboundRequiredField)?;
        required(envelope.aggregate_type, ReasonCode::InboundRequiredField)?;
        envelope.aggregate_id.ok_or_else(missing)?;
        let occurred_at = envelope.occurred_at.ok_or_else(missing)?;
        if envelope.event_version != Some(1) {
            return Err(NucleoException::new(ReasonCode::UnsupportedEventVersion));
        }
        if routing_key != RoutingKeyCatalog::NUCLEO_RETRIEVE_V1
            || envelope.routing_key.as_deref() != Some(routing_key)
        {
            return Err(NucleoException::new(ReasonCode::UnsupportedRoutingKey));
        }
        if exchange != ExchangeCatalog::NUCLEO_COMMAND
            || envelope.exchange_name.as_deref() != Some(exchange)
        {
            return Err(NucleoException::new(ReasonCode::ValidationError));
        }
        if envelope
            .user_agent
            .as_ref()
            .is_some_and(|value| value.chars().count() > 255)
            || envelope
                .origin_ip
                .as_ref()
                .is_some_and(|value| value.chars().count() > 255)
        {
            return Err(NucleoException::new(ReasonCode::ValidationError));
        }
        let payload = envelope.payload.ok_or_else(missing)?;
        let nucleo_id = payload.nucleo_id.ok_or_else(missing)?;
        let source_event_id = payload.source_event_id.ok_or_else(missing)?;
        let callback_exchange =
            required(payload.callback_exchange, ReasonCode::InboundRequiredField)?;
        if callback_exchange.len() > 255 {
            return Err(NucleoException::new(ReasonCode::ValidationError));
        }
        required(
            payload.callback_routing_key,
            ReasonCode::InboundRequiredField,
        )?;
        Ok(NucleoRetrieveCommand {
            event_id,
            correlation_id,
            source_event_id,
            nucleo_id,
            callback_exchange,
            actor_id: envelope.actor_id,
            user_agent: envelope.user_agent,
            origin_ip: envelope.origin_ip,
            occurred_at,
        })
    }
}
