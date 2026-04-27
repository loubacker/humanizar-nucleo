package com.humanizar.nucleo.application.service.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.usecase.update.UpdateAbordagemUseCase;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@Transactional
public class AbordagemServiceUpdate {

    private final UpdateAbordagemUseCase updateAbordagemUseCase;
    private final AbordagemRetrieveMapper abordagemRetrieveMapper;

    public AbordagemServiceUpdate(UpdateAbordagemUseCase updateAbordagemUseCase,
                                  AbordagemRetrieveMapper abordagemRetrieveMapper) {
        this.updateAbordagemUseCase = updateAbordagemUseCase;
        this.abordagemRetrieveMapper = abordagemRetrieveMapper;
    }

    public AbordagemRetrieveDTO execute(UUID abordagemId, AbordagemUpdateDTO abordagemUpdateDTO) {
        var abordagem = updateAbordagemUseCase.execute(abordagemId, abordagemUpdateDTO);
        return abordagemRetrieveMapper.toDTO(abordagem);
    }
}
