package com.humanizar.nucleo.application.mapper;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.domain.model.Abordagem;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class AbordagemCreateMapper {

    public Abordagem toDomain(AbordagemRegisterDTO abordagemRegisterDTO) {
        return Abordagem.builder()
                .nome(abordagemRegisterDTO.nome())
                .descricao(abordagemRegisterDTO.descricao())
                .nucleoId(abordagemRegisterDTO.nucleoId())
                .build();
    }

    public Abordagem toUpdatedDomain(UUID abordagemId,
                                     AbordagemUpdateDTO abordagemUpdateDTO,
                                     Abordagem abordagemAtual) {
        return Abordagem.builder()
                .id(abordagemId)
                .nome(abordagemUpdateDTO.nome())
                .descricao(abordagemUpdateDTO.descricao())
                .nucleoId(abordagemAtual.getNucleoId())
                .createdAt(abordagemAtual.getCreatedAt())
                .updatedAt(abordagemAtual.getUpdatedAt())
                .build();
    }
}
