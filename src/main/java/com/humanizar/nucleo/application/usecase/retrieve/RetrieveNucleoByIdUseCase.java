package com.humanizar.nucleo.application.usecase.retrieve;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class RetrieveNucleoByIdUseCase {

    private final NucleoPort nucleoPort;

    public RetrieveNucleoByIdUseCase(NucleoPort nucleoPort) {
        this.nucleoPort = nucleoPort;
    }

    public Nucleo execute(UUID nucleoId) {
        return nucleoPort.findById(nucleoId)
                .orElseThrow(() -> new NucleoException(ReasonCode.NUCLEO_NOT_FOUND));
    }
}
