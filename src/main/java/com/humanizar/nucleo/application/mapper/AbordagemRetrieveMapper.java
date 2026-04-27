package com.humanizar.nucleo.application.mapper;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.domain.model.Abordagem;
import org.springframework.stereotype.Component;

@Component
public class AbordagemRetrieveMapper {

    public AbordagemRetrieveDTO toDTO(Abordagem abordagem) {
        return new AbordagemRetrieveDTO(
                abordagem.getId(),
                abordagem.getNome(),
                abordagem.getDescricao(),
                abordagem.getNucleoId(),
                abordagem.getCreatedAt(),
                abordagem.getUpdatedAt());
    }
}
