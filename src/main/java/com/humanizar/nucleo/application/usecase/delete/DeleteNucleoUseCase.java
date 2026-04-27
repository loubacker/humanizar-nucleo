package com.humanizar.nucleo.application.usecase.delete;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class DeleteNucleoUseCase {

    private final NucleoPort nucleoPort;

    public DeleteNucleoUseCase(NucleoPort nucleoPort) {
        this.nucleoPort = nucleoPort;
    }

    public void execute(UUID nucleoId) {
        try {
            nucleoPort.deleteById(nucleoId);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.NUCLEO_DELETE_FAILED, runtimeException);
        }
    }
}
