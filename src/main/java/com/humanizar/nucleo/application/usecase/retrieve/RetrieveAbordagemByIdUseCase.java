package com.humanizar.nucleo.application.usecase.retrieve;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class RetrieveAbordagemByIdUseCase {

    private final AbordagemPort abordagemPort;

    public RetrieveAbordagemByIdUseCase(AbordagemPort abordagemPort) {
        this.abordagemPort = abordagemPort;
    }

    public Abordagem execute(UUID abordagemId) {
        return abordagemPort.findById(abordagemId)
                .orElseThrow(() -> new NucleoException(ReasonCode.ABORDAGEM_NOT_FOUND));
    }
}
