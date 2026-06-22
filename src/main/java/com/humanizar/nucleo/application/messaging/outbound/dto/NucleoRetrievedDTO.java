package com.humanizar.nucleo.application.messaging.outbound.dto;

import java.util.UUID;

public record NucleoRetrievedDTO(
        UUID requestEventId,
        UUID sourceEventId,
        UUID nucleoId,
        String nome) {
}
