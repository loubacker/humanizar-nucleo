package com.humanizar.nucleo.application.messaging.outbound.dto;

import java.util.UUID;

public record NucleoRejectedDTO(
        UUID requestEventId,
        UUID sourceEventId,
        UUID nucleoId,
        String reasonCode,
        String message) {
}
