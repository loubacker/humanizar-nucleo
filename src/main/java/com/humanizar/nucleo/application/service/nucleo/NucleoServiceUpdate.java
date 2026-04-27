package com.humanizar.nucleo.application.service.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.application.mapper.NucleoRetrieveMapper;
import com.humanizar.nucleo.application.usecase.update.UpdateNucleoUseCase;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@Transactional
public class NucleoServiceUpdate {

    private final UpdateNucleoUseCase updateNucleoUseCase;
    private final AbordagemPort abordagemPort;
    private final NucleoRetrieveMapper nucleoRetrieveMapper;

    public NucleoServiceUpdate(UpdateNucleoUseCase updateNucleoUseCase,
                               AbordagemPort abordagemPort,
                               NucleoRetrieveMapper nucleoRetrieveMapper) {
        this.updateNucleoUseCase = updateNucleoUseCase;
        this.abordagemPort = abordagemPort;
        this.nucleoRetrieveMapper = nucleoRetrieveMapper;
    }

    public NucleoRetrieveDTO execute(UUID nucleoId, NucleoUpdateDTO nucleoUpdateDTO) {
        var nucleo = updateNucleoUseCase.execute(nucleoId, nucleoUpdateDTO);
        var abordagens = abordagemPort.findByNucleoId(nucleoId);
        return nucleoRetrieveMapper.toDTO(nucleo, abordagens);
    }
}
