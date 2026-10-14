#![forbid(unsafe_code)]

use humanizar_nucleo::application::catalog::{ExchangeCatalog, RoutingKeyCatalog};
use humanizar_nucleo::application::messaging::inbound::mapper::NucleoCommandMapper;
use humanizar_nucleo::domain::model::enums::ReasonCode;
use serde_json::{Value, json};
use uuid::Uuid;

#[test]
fn notification_context_does_not_replace_the_nucleo_requested_in_payload() {
    let envelope = notification_envelope();
    assert_ne!(envelope["aggregateId"], envelope["payload"]["nucleoId"]);
    let command = NucleoCommandMapper::parse(
        &serde_json::to_vec(&envelope).expect("notification envelope"),
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
    )
    .expect("notification context is independent from the lookup target");
    assert_eq!(
        command.event_id.to_string(),
        envelope["eventId"].as_str().expect("event ID")
    );
    assert_eq!(
        command.correlation_id.to_string(),
        envelope["correlationId"].as_str().expect("correlation ID")
    );
    assert_eq!(
        command.nucleo_id.to_string(),
        envelope["payload"]["nucleoId"].as_str().expect("nucleo ID")
    );
    assert_eq!(
        command.source_event_id.to_string(),
        envelope["payload"]["sourceEventId"]
            .as_str()
            .expect("source event ID")
    );
    assert_eq!(command.callback_exchange, ExchangeCatalog::NUCLEO_EVENT);
    assert_eq!(
        command.actor_id.expect("actor ID").to_string(),
        envelope["actorId"].as_str().expect("envelope actor ID")
    );
    assert_eq!(command.user_agent.as_deref(), Some("fixture-agent"));
    assert_eq!(command.origin_ip.as_deref(), Some("127.0.0.1"));
    assert_eq!(command.occurred_at.to_string(), "2026-10-13 15:36:44");
}

#[test]
fn nucleo_context_can_also_identify_the_lookup_target() {
    let mut envelope = notification_envelope();
    envelope["aggregateType"] = json!("Nucleo");
    envelope["aggregateId"] = envelope["payload"]["nucleoId"].clone();
    let command = NucleoCommandMapper::parse(
        &serde_json::to_vec(&envelope).expect("nucleo envelope"),
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
    )
    .expect("nucleo context");
    assert_eq!(
        command.nucleo_id.to_string(),
        envelope["aggregateId"].as_str().expect("aggregate ID")
    );
}

#[test]
fn aggregate_context_remains_required_and_must_be_a_uuid() {
    let mut missing = notification_envelope();
    missing
        .as_object_mut()
        .expect("envelope object")
        .remove("aggregateId");
    assert_reason(
        &missing,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::InboundRequiredField,
    );
    let mut null = notification_envelope();
    null["aggregateId"] = Value::Null;
    assert_reason(
        &null,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::InboundRequiredField,
    );
    let mut invalid = notification_envelope();
    invalid["aggregateId"] = json!("not-a-uuid");
    assert_reason(
        &invalid,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::InboundParseError,
    );
}

#[test]
fn notification_payload_fields_remain_required() {
    for field in [
        "sourceEventId",
        "nucleoId",
        "callbackExchange",
        "callbackRoutingKey",
    ] {
        let mut envelope = notification_envelope();
        envelope["payload"]
            .as_object_mut()
            .expect("payload object")
            .remove(field);
        assert_reason(
            &envelope,
            ExchangeCatalog::NUCLEO_COMMAND,
            RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
            ReasonCode::InboundRequiredField,
        );
    }
}

#[test]
fn broker_destination_and_version_remain_validated() {
    let envelope = notification_envelope();
    assert_reason(
        &envelope,
        ExchangeCatalog::NUCLEO_COMMAND,
        "unsupported.routing.key",
        ReasonCode::UnsupportedRoutingKey,
    );
    assert_reason(
        &envelope,
        "unsupported.exchange",
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::ValidationError,
    );
    let mut wrong_routing = envelope.clone();
    wrong_routing["routingKey"] = json!("unsupported.routing.key");
    assert_reason(
        &wrong_routing,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::UnsupportedRoutingKey,
    );
    let mut wrong_exchange = envelope.clone();
    wrong_exchange["exchangeName"] = json!("unsupported.exchange");
    assert_reason(
        &wrong_exchange,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::ValidationError,
    );
    let mut wrong_version = envelope;
    wrong_version["eventVersion"] = json!(2);
    assert_reason(
        &wrong_version,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::UnsupportedEventVersion,
    );
}

#[test]
fn context_and_callback_size_limits_remain_validated() {
    for field in ["userAgent", "originIp"] {
        let mut envelope = notification_envelope();
        envelope[field] = json!("a".repeat(256));
        assert_reason(
            &envelope,
            ExchangeCatalog::NUCLEO_COMMAND,
            RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
            ReasonCode::ValidationError,
        );
    }
    let mut callback = notification_envelope();
    callback["payload"]["callbackExchange"] = json!("é".repeat(128));
    assert_reason(
        &callback,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        ReasonCode::ValidationError,
    );
    let oversized = vec![b' '; 2 * 1024 * 1024 + 1];
    let error = NucleoCommandMapper::parse(
        &oversized,
        ExchangeCatalog::NUCLEO_COMMAND,
        RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
    )
    .err()
    .expect("oversized body rejection");
    assert_eq!(error.reason_code(), ReasonCode::ValidationError);
}

fn notification_envelope() -> Value {
    json!({
        "eventId": Uuid::new_v4(),
        "correlationId": Uuid::new_v4(),
        "producerService": "humanizar-notification",
        "exchangeName": ExchangeCatalog::NUCLEO_COMMAND,
        "routingKey": RoutingKeyCatalog::NUCLEO_RETRIEVE_V1,
        "aggregateType": "NotificationRequest",
        "aggregateId": Uuid::new_v4(),
        "eventVersion": 1,
        "occurredAt": "2026-10-13T15:36:44",
        "actorId": Uuid::new_v4(),
        "userAgent": "fixture-agent",
        "originIp": "127.0.0.1",
        "payload": {
            "sourceEventId": Uuid::new_v4(),
            "nucleoId": Uuid::new_v4(),
            "callbackExchange": ExchangeCatalog::NUCLEO_EVENT,
            "callbackRoutingKey": RoutingKeyCatalog::NUCLEO_RETRIEVED_V1
        }
    })
}

fn assert_reason(envelope: &Value, exchange: &str, routing_key: &str, expected: ReasonCode) {
    let error = NucleoCommandMapper::parse(
        &serde_json::to_vec(envelope).expect("envelope"),
        exchange,
        routing_key,
    )
    .err()
    .expect("invalid command rejection");
    assert_eq!(error.reason_code(), expected);
}
