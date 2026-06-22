package com.humanizar.nucleo.application.messaging.inbound.dto;

import java.util.UUID;

public record NucleoRetrieveCommandDTO(
        UUID sourceEventId,
        UUID nucleoId,
        String callbackExchange,
        String callbackRoutingKey) {
}
