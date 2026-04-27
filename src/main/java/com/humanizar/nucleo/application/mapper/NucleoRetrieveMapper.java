package com.humanizar.nucleo.application.mapper;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.Nucleo;
import org.springframework.stereotype.Component;

import java.util.List;

@Component
public class NucleoRetrieveMapper {

    private final AbordagemRetrieveMapper abordagemRetrieveMapper;

    public NucleoRetrieveMapper(AbordagemRetrieveMapper abordagemRetrieveMapper) {
        this.abordagemRetrieveMapper = abordagemRetrieveMapper;
    }

    public NucleoRetrieveDTO toDTO(Nucleo nucleo, List<Abordagem> abordagens) {
        List<AbordagemRetrieveDTO> abordagemRetrieveDTOs = abordagens.stream()
                .map(abordagemRetrieveMapper::toDTO)
                .toList();

        return new NucleoRetrieveDTO(
                nucleo.getId(),
                nucleo.getNome(),
                nucleo.getDescricao(),
                abordagemRetrieveDTOs,
                nucleo.getCreatedAt(),
                nucleo.getUpdatedAt());
    }
}
