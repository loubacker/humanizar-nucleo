package com.humanizar.nucleoservice.application.mapper;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.mapper.NucleoRetrieveMapper;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.Nucleo;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.time.LocalDateTime;
import java.util.List;
import java.util.UUID;

class NucleoRetrieveMapperTest {

    private final NucleoRetrieveMapper nucleoRetrieveMapper =
            new NucleoRetrieveMapper(new AbordagemRetrieveMapper());

    @Test
    void shouldMapDomainAndAbordagensToRetrieveDto() {
        UUID nucleoId = UUID.randomUUID();
        UUID abordagemId = UUID.randomUUID();
        LocalDateTime createdAt = LocalDateTime.now().minusDays(1);
        LocalDateTime updatedAt = LocalDateTime.now();
        Nucleo nucleo = Nucleo.builder()
                .id(nucleoId)
                .nome("Infantil")
                .descricao("Descricao")
                .createdAt(createdAt)
                .updatedAt(updatedAt)
                .build();
        Abordagem abordagem = Abordagem.builder()
                .id(abordagemId)
                .nome("ABA")
                .descricao("Descricao abordagem")
                .nucleoId(nucleoId)
                .createdAt(createdAt)
                .updatedAt(updatedAt)
                .build();

        NucleoRetrieveDTO nucleoRetrieveDTO = nucleoRetrieveMapper.toDTO(nucleo, List.of(abordagem));

        Assertions.assertEquals(nucleoId, nucleoRetrieveDTO.id());
        Assertions.assertEquals("Infantil", nucleoRetrieveDTO.nome());
        Assertions.assertEquals("Descricao", nucleoRetrieveDTO.descricao());
        Assertions.assertEquals(1, nucleoRetrieveDTO.abordagens().size());
        Assertions.assertEquals(abordagemId, nucleoRetrieveDTO.abordagens().getFirst().id());
        Assertions.assertEquals(createdAt, nucleoRetrieveDTO.createdAt());
        Assertions.assertEquals(updatedAt, nucleoRetrieveDTO.updatedAt());
    }
}
