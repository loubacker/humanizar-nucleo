package com.humanizar.nucleo.application.dto.abordagem;

import java.time.LocalDateTime;
import java.util.UUID;

public record AbordagemRetrieveDTO(UUID id,
                                   String nome,
                                   String descricao,
                                   UUID nucleoId,
                                   LocalDateTime createdAt,
                                   LocalDateTime updatedAt) {
}
