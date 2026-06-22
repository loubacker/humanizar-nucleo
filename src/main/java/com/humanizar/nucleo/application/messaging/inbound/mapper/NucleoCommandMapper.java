package com.humanizar.nucleo.application.messaging.inbound.mapper;

import org.springframework.stereotype.Component;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.humanizar.nucleo.application.catalog.RoutingKeyCatalog;
import com.humanizar.nucleo.application.messaging.inbound.dto.InboundEnvelopeDTO;
import com.humanizar.nucleo.application.messaging.inbound.dto.NucleoRetrieveCommandDTO;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;

@Component
public class NucleoCommandMapper {

    private final ObjectMapper objectMapper;

    public NucleoCommandMapper(ObjectMapper objectMapper) {
        this.objectMapper = objectMapper;
    }

    public NucleoRetrieveCommandDTO toCommand(InboundEnvelopeDTO<Object> envelope) {
        String routingKey = envelope.routingKey();
        if (!RoutingKeyCatalog.NUCLEO_RETRIEVE_V1.equals(routingKey)) {
            throw unsupported(envelope, routingKey);
        }

        NucleoRetrieveCommandDTO command = objectMapper.convertValue(
                envelope.payload(),
                NucleoRetrieveCommandDTO.class);
        requireField(command.sourceEventId(), "sourceEventId", envelope);
        requireField(command.nucleoId(), "nucleoId", envelope);
        requireNotBlank(command.callbackExchange(), "callbackExchange", envelope);
        requireNotBlank(command.callbackRoutingKey(), "callbackRoutingKey", envelope);
        return command;
    }

    private NucleoException unsupported(InboundEnvelopeDTO<?> envelope, String routingKey) {
        return new NucleoException(
                ReasonCode.UNSUPPORTED_ROUTING_KEY,
                "Routing key nao suportada: " + routingKey,
                correlationId(envelope));
    }

    private void requireField(Object value, String fieldName, InboundEnvelopeDTO<?> envelope) {
        if (value == null) {
            throw new NucleoException(
                    ReasonCode.INBOUND_REQUIRED_FIELD,
                    "Campo obrigatorio ausente: " + fieldName,
                    correlationId(envelope));
        }
    }

    private void requireNotBlank(String value, String fieldName, InboundEnvelopeDTO<?> envelope) {
        if (value == null || value.isBlank()) {
            throw new NucleoException(
                    ReasonCode.INBOUND_REQUIRED_FIELD,
                    "Campo obrigatorio ausente: " + fieldName,
                    correlationId(envelope));
        }
    }

    private String correlationId(InboundEnvelopeDTO<?> envelope) {
        if (envelope == null || envelope.correlationId() == null) {
            return null;
        }
        return envelope.correlationId().toString();
    }
}
