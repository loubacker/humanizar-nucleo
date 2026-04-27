package com.humanizar.nucleoservice.application.mapper;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.application.mapper.NucleoCreateMapper;
import com.humanizar.nucleo.domain.model.Nucleo;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.time.LocalDateTime;
import java.util.UUID;

class NucleoCreateMapperTest {

    private final NucleoCreateMapper nucleoCreateMapper = new NucleoCreateMapper();

    @Test
    void shouldMapRegisterDtoToDomain() {
        NucleoRegisterDTO nucleoRegisterDTO = new NucleoRegisterDTO("Infantil", "Atendimento infantil");

        Nucleo nucleo = nucleoCreateMapper.toDomain(nucleoRegisterDTO);

        Assertions.assertNull(nucleo.getId());
        Assertions.assertEquals("Infantil", nucleo.getNome());
        Assertions.assertEquals("Atendimento infantil", nucleo.getDescricao());
    }

    @Test
    void shouldMapUpdateDtoToDomainPreservingIdentityAndTimestamps() {
        UUID nucleoId = UUID.randomUUID();
        LocalDateTime createdAt = LocalDateTime.now().minusDays(1);
        LocalDateTime updatedAt = LocalDateTime.now();
        Nucleo nucleoAtual = Nucleo.builder()
                .id(nucleoId)
                .nome("Anterior")
                .descricao("Descricao anterior")
                .createdAt(createdAt)
                .updatedAt(updatedAt)
                .build();

        NucleoUpdateDTO nucleoUpdateDTO = new NucleoUpdateDTO("Novo", "Descricao nova");

        Nucleo nucleoAtualizado = nucleoCreateMapper.toUpdatedDomain(nucleoId, nucleoUpdateDTO, nucleoAtual);

        Assertions.assertEquals(nucleoId, nucleoAtualizado.getId());
        Assertions.assertEquals("Novo", nucleoAtualizado.getNome());
        Assertions.assertEquals("Descricao nova", nucleoAtualizado.getDescricao());
        Assertions.assertEquals(createdAt, nucleoAtualizado.getCreatedAt());
        Assertions.assertEquals(updatedAt, nucleoAtualizado.getUpdatedAt());
    }
}
