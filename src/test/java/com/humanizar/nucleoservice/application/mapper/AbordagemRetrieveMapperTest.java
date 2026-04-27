package com.humanizar.nucleoservice.application.mapper;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.domain.model.Abordagem;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.time.LocalDateTime;
import java.util.UUID;

class AbordagemRetrieveMapperTest {

    private final AbordagemRetrieveMapper abordagemRetrieveMapper = new AbordagemRetrieveMapper();

    @Test
    void shouldMapDomainToRetrieveDto() {
        UUID abordagemId = UUID.randomUUID();
        UUID nucleoId = UUID.randomUUID();
        LocalDateTime createdAt = LocalDateTime.now().minusDays(1);
        LocalDateTime updatedAt = LocalDateTime.now();
        Abordagem abordagem = Abordagem.builder()
                .id(abordagemId)
                .nome("ABA")
                .descricao("Descricao")
                .nucleoId(nucleoId)
                .createdAt(createdAt)
                .updatedAt(updatedAt)
                .build();

        AbordagemRetrieveDTO abordagemRetrieveDTO = abordagemRetrieveMapper.toDTO(abordagem);

        Assertions.assertEquals(abordagemId, abordagemRetrieveDTO.id());
        Assertions.assertEquals("ABA", abordagemRetrieveDTO.nome());
        Assertions.assertEquals("Descricao", abordagemRetrieveDTO.descricao());
        Assertions.assertEquals(nucleoId, abordagemRetrieveDTO.nucleoId());
        Assertions.assertEquals(createdAt, abordagemRetrieveDTO.createdAt());
        Assertions.assertEquals(updatedAt, abordagemRetrieveDTO.updatedAt());
    }
}
