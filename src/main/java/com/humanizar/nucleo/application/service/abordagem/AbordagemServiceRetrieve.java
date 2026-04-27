package com.humanizar.nucleo.application.service.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.usecase.retrieve.RetrieveAbordagemByIdUseCase;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@Transactional(readOnly = true)
public class AbordagemServiceRetrieve {

    private final RetrieveAbordagemByIdUseCase retrieveAbordagemByIdUseCase;
    private final AbordagemRetrieveMapper abordagemRetrieveMapper;

    public AbordagemServiceRetrieve(RetrieveAbordagemByIdUseCase retrieveAbordagemByIdUseCase,
                                    AbordagemRetrieveMapper abordagemRetrieveMapper) {
        this.retrieveAbordagemByIdUseCase = retrieveAbordagemByIdUseCase;
        this.abordagemRetrieveMapper = abordagemRetrieveMapper;
    }

    public AbordagemRetrieveDTO execute(UUID abordagemId) {
        var abordagem = retrieveAbordagemByIdUseCase.execute(abordagemId);
        return abordagemRetrieveMapper.toDTO(abordagem);
    }
}
