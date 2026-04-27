package com.humanizar.nucleo.application.service.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.usecase.create.CreateAbordagemUseCase;
import com.humanizar.nucleo.domain.model.Abordagem;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

@Service
@Transactional
public class AbordagemServiceCreate {

    private final CreateAbordagemUseCase createAbordagemUseCase;
    private final AbordagemRetrieveMapper abordagemRetrieveMapper;

    public AbordagemServiceCreate(CreateAbordagemUseCase createAbordagemUseCase,
                                  AbordagemRetrieveMapper abordagemRetrieveMapper) {
        this.createAbordagemUseCase = createAbordagemUseCase;
        this.abordagemRetrieveMapper = abordagemRetrieveMapper;
    }

    public AbordagemRetrieveDTO execute(AbordagemRegisterDTO abordagemRegisterDTO) {
        Abordagem abordagem = createAbordagemUseCase.execute(abordagemRegisterDTO);
        return abordagemRetrieveMapper.toDTO(abordagem);
    }
}
