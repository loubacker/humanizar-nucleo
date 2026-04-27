package com.humanizar.nucleo.application.usecase.retrieve;

import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

import java.util.List;

@Component
public class RetrieveAllNucleosUseCase {

    private final NucleoPort nucleoPort;

    public RetrieveAllNucleosUseCase(NucleoPort nucleoPort) {
        this.nucleoPort = nucleoPort;
    }

    public List<Nucleo> executeWithoutAbordagens() {
        return nucleoPort.findAll();
    }

    public List<Nucleo> executeWithAbordagens() {
        return nucleoPort.findAll();
    }
}
