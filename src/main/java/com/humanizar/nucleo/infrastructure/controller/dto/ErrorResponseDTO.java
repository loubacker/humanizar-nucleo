package com.humanizar.nucleo.infrastructure.controller.dto;

import java.time.LocalDateTime;

public record ErrorResponseDTO(
        LocalDateTime timestamp,
        int status,
        String error,
        String reasonCode,
        String message,
        String path) {

    public ErrorResponseDTO(int status, String error, String reasonCode, String message, String path) {
        this(LocalDateTime.now(), status, error, reasonCode, message, path);
    }
}
