package com.humanizar.nucleo.application.mapper;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.domain.model.Nucleo;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class NucleoCreateMapper {

    public Nucleo toDomain(NucleoRegisterDTO nucleoRegisterDTO) {
        return Nucleo.builder()
                .nome(nucleoRegisterDTO.nome())
                .descricao(nucleoRegisterDTO.descricao())
                .build();
    }

    public Nucleo toUpdatedDomain(UUID nucleoId, NucleoUpdateDTO nucleoUpdateDTO, Nucleo nucleoAtual) {
        return Nucleo.builder()
                .id(nucleoId)
                .nome(nucleoUpdateDTO.nome())
                .descricao(nucleoUpdateDTO.descricao())
                .createdAt(nucleoAtual.getCreatedAt())
                .updatedAt(nucleoAtual.getUpdatedAt())
                .build();
    }
}
