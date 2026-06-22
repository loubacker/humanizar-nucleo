package com.humanizar.nucleo.application.messaging.inbound.mapper;

import java.io.IOException;

import org.springframework.stereotype.Component;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.humanizar.nucleo.application.messaging.inbound.dto.InboundEnvelopeDTO;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;

@Component
public class EnvelopeInboundMapper {

    private final ObjectMapper objectMapper;

    public EnvelopeInboundMapper(ObjectMapper objectMapper) {
        this.objectMapper = objectMapper;
    }

    public InboundEnvelopeDTO<Object> parseEnvelope(byte[] body) {
        try {
            return objectMapper.readValue(body, new TypeReference<>() {
            });
        } catch (IOException exception) {
            throw new NucleoException(ReasonCode.INBOUND_PARSE_ERROR, "Falha ao parsear envelope inbound.");
        }
    }

    public void validate(InboundEnvelopeDTO<?> envelope) {
        requireField(envelope, "envelope", null);
        String correlationId = correlationIdAsString(envelope);
        requireField(envelope.eventId(), "eventId", correlationId);
        requireField(envelope.correlationId(), "correlationId", null);
        requireNotBlank(envelope.producerService(), "producerService", correlationId);
        requireNotBlank(envelope.aggregateType(), "aggregateType", correlationId);
        requireField(envelope.aggregateId(), "aggregateId", correlationId);
        requireField(envelope.occurredAt(), "occurredAt", correlationId);
        requireField(envelope.payload(), "payload", correlationId);
    }

    public String correlationIdAsString(InboundEnvelopeDTO<?> envelope) {
        if (envelope == null || envelope.correlationId() == null) {
            return null;
        }
        return envelope.correlationId().toString();
    }

    private void requireField(Object value, String fieldName, String correlationId) {
        if (value == null) {
            throw new NucleoException(
                    ReasonCode.INBOUND_REQUIRED_FIELD,
                    "Campo obrigatório ausente: " + fieldName,
                    correlationId);
        }
    }

    private void requireNotBlank(String value, String fieldName, String correlationId) {
        if (value == null || value.isBlank()) {
            throw new NucleoException(
                    ReasonCode.INBOUND_REQUIRED_FIELD,
                    "Campo obrigatório ausente: " + fieldName,
                    correlationId);
        }
    }
}
