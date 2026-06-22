package com.humanizar.nucleo.application.messaging.inbound.dto;

import java.time.LocalDateTime;
import java.util.UUID;

public record InboundEnvelopeDTO<T>(
        UUID eventId,
        UUID correlationId,
        String producerService,
        String exchangeName,
        String routingKey,
        String aggregateType,
        UUID aggregateId,
        Integer eventVersion,
        LocalDateTime occurredAt,
        UUID actorId,
        String userAgent,
        String originIp,
        T payload) {
}
