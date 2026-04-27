package com.humanizar.nucleo.application.service.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.mapper.NucleoRetrieveMapper;
import com.humanizar.nucleo.application.usecase.create.CreateNucleoUseCase;
import com.humanizar.nucleo.domain.model.Nucleo;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;

@Service
@Transactional
public class NucleoServiceCreate {

    private final CreateNucleoUseCase createNucleoUseCase;
    private final NucleoRetrieveMapper nucleoRetrieveMapper;

    public NucleoServiceCreate(CreateNucleoUseCase createNucleoUseCase,
                               NucleoRetrieveMapper nucleoRetrieveMapper) {
        this.createNucleoUseCase = createNucleoUseCase;
        this.nucleoRetrieveMapper = nucleoRetrieveMapper;
    }

    public NucleoRetrieveDTO execute(NucleoRegisterDTO nucleoRegisterDTO) {
        Nucleo nucleo = createNucleoUseCase.execute(nucleoRegisterDTO);
        return nucleoRetrieveMapper.toDTO(nucleo, List.of());
    }
}
