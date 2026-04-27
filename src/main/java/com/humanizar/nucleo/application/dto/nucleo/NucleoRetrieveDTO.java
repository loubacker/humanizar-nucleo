package com.humanizar.nucleo.application.dto.nucleo;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

public record NucleoRetrieveDTO(UUID id,
                                String nome,
                                String descricao,
                                List<AbordagemRetrieveDTO> abordagens,
                                LocalDateTime createdAt,
                                LocalDateTime updatedAt) {
}
