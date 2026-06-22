package com.humanizar.nucleo.application.messaging.outbound.mapper;

import java.time.LocalDateTime;
import java.util.Objects;
import java.util.UUID;

import org.springframework.stereotype.Component;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.OutboxEvent;
import com.humanizar.nucleo.domain.model.enums.OutboxStatus;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;

@Component
public class OutboxEventMapper {

    private static final String PRODUCER_SERVICE = "humanizar-nucleo";
    private static final short EVENT_VERSION = 1;
    private static final int DEFAULT_MAX_ATTEMPTS = 5;

    private final ObjectMapper objectMapper;

    public OutboxEventMapper(ObjectMapper objectMapper) {
        this.objectMapper = objectMapper;
    }

    public OutboxEvent toOutboxEvent(String exchangeName,
            String routingKey,
            String aggregateType,
            UUID aggregateId,
            UUID eventId,
            UUID correlationId,
            Object payload,
            UUID actorId,
            String userAgent,
            String originIp) {
        String correlationAsString = correlationId != null ? correlationId.toString() : null;
        requireText(exchangeName, "exchangeName", correlationAsString);
        requireText(routingKey, "routingKey", correlationAsString);
        requireText(aggregateType, "aggregateType", correlationAsString);
        requireNonNull(aggregateId, "aggregateId", correlationAsString);
        requireNonNull(eventId, "eventId", correlationAsString);
        requireNonNull(correlationId, "correlationId", correlationAsString);
        requireNonNull(payload, "payload", correlationAsString);

        return OutboxEvent.builder()
                .eventId(eventId)
                .correlationId(correlationId)
                .producerService(PRODUCER_SERVICE)
                .exchangeName(exchangeName)
                .routingKey(routingKey)
                .aggregateType(aggregateType)
                .aggregateId(aggregateId)
                .eventVersion(EVENT_VERSION)
                .payload(serialize(payload))
                .actorId(actorId)
                .userAgent(userAgent)
                .originIp(originIp)
                .status(OutboxStatus.NEW)
                .attemptCount(0)
                .maxAttempts(DEFAULT_MAX_ATTEMPTS)
                .nextRetryAt(LocalDateTime.now())
                .build();
    }

    private void requireText(String value, String fieldName, String correlationId) {
        if (value == null || value.isBlank()) {
            throw new NucleoException(
                    ReasonCode.VALIDATION_ERROR,
                    "Campo obrigatório ausente no outbox: " + fieldName,
                    correlationId);
        }
    }

    private <T> void requireNonNull(T value, String fieldName, String correlationId) {
        if (Objects.isNull(value)) {
            throw new NucleoException(
                    ReasonCode.VALIDATION_ERROR,
                    "Campo obrigatório ausente no outbox: " + fieldName,
                    correlationId);
        }
    }

    private String serialize(Object value) {
        try {
            return objectMapper.writeValueAsString(value);
        } catch (JsonProcessingException exception) {
            throw new NucleoException(ReasonCode.PERSISTENCE_FAILURE, "Falha ao serializar payload para outbox.");
        }
    }
}
