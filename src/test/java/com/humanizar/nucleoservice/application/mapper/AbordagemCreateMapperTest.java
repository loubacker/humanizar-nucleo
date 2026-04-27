package com.humanizar.nucleoservice.application.mapper;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.application.mapper.AbordagemCreateMapper;
import com.humanizar.nucleo.domain.model.Abordagem;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.time.LocalDateTime;
import java.util.UUID;

class AbordagemCreateMapperTest {

    private final AbordagemCreateMapper abordagemCreateMapper = new AbordagemCreateMapper();

    @Test
    void shouldMapRegisterDtoToDomain() {
        UUID nucleoId = UUID.randomUUID();
        AbordagemRegisterDTO abordagemRegisterDTO = new AbordagemRegisterDTO("ABA", "Abordagem comportamental", nucleoId);

        Abordagem abordagem = abordagemCreateMapper.toDomain(abordagemRegisterDTO);

        Assertions.assertEquals("ABA", abordagem.getNome());
        Assertions.assertEquals("Abordagem comportamental", abordagem.getDescricao());
        Assertions.assertEquals(nucleoId, abordagem.getNucleoId());
    }

    @Test
    void shouldMapUpdateDtoToDomainPreservingNucleoIdAndTimestamps() {
        UUID abordagemId = UUID.randomUUID();
        UUID nucleoId = UUID.randomUUID();
        LocalDateTime createdAt = LocalDateTime.now().minusDays(1);
        LocalDateTime updatedAt = LocalDateTime.now();
        Abordagem abordagemAtual = Abordagem.builder()
                .id(abordagemId)
                .nome("Anterior")
                .descricao("Descricao anterior")
                .nucleoId(nucleoId)
                .createdAt(createdAt)
                .updatedAt(updatedAt)
                .build();

        AbordagemUpdateDTO abordagemUpdateDTO = new AbordagemUpdateDTO("Nova", "Descricao nova");

        Abordagem abordagemAtualizada =
                abordagemCreateMapper.toUpdatedDomain(abordagemId, abordagemUpdateDTO, abordagemAtual);

        Assertions.assertEquals(abordagemId, abordagemAtualizada.getId());
        Assertions.assertEquals("Nova", abordagemAtualizada.getNome());
        Assertions.assertEquals("Descricao nova", abordagemAtualizada.getDescricao());
        Assertions.assertEquals(nucleoId, abordagemAtualizada.getNucleoId());
        Assertions.assertEquals(createdAt, abordagemAtualizada.getCreatedAt());
        Assertions.assertEquals(updatedAt, abordagemAtualizada.getUpdatedAt());
    }
}
