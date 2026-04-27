package com.humanizar.nucleo.application.service.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.mapper.NucleoRetrieveMapper;
import com.humanizar.nucleo.application.usecase.retrieve.RetrieveAllNucleosUseCase;
import com.humanizar.nucleo.application.usecase.retrieve.RetrieveNucleoByIdUseCase;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Service
@Transactional(readOnly = true)
public class NucleoServiceRetrieve {

    private final RetrieveNucleoByIdUseCase retrieveNucleoByIdUseCase;
    private final RetrieveAllNucleosUseCase retrieveAllNucleosUseCase;
    private final AbordagemPort abordagemPort;
    private final NucleoRetrieveMapper nucleoRetrieveMapper;

    public NucleoServiceRetrieve(RetrieveNucleoByIdUseCase retrieveNucleoByIdUseCase,
                                 RetrieveAllNucleosUseCase retrieveAllNucleosUseCase,
                                 AbordagemPort abordagemPort,
                                 NucleoRetrieveMapper nucleoRetrieveMapper) {
        this.retrieveNucleoByIdUseCase = retrieveNucleoByIdUseCase;
        this.retrieveAllNucleosUseCase = retrieveAllNucleosUseCase;
        this.abordagemPort = abordagemPort;
        this.nucleoRetrieveMapper = nucleoRetrieveMapper;
    }

    public List<NucleoRetrieveDTO> executeWithoutAbordagens() {
        return retrieveAllNucleosUseCase.executeWithoutAbordagens().stream()
                .map(nucleo -> nucleoRetrieveMapper.toDTO(nucleo, List.of()))
                .toList();
    }

    public List<NucleoRetrieveDTO> executeWithAbordagens() {
        return retrieveAllNucleosUseCase.executeWithAbordagens().stream()
                .map(nucleo -> nucleoRetrieveMapper.toDTO(nucleo, abordagemPort.findByNucleoId(nucleo.getId())))
                .toList();
    }

    public NucleoRetrieveDTO executeById(UUID nucleoId) {
        var nucleo = retrieveNucleoByIdUseCase.execute(nucleoId);
        var abordagens = abordagemPort.findByNucleoId(nucleoId);
        return nucleoRetrieveMapper.toDTO(nucleo, abordagens);
    }
}
