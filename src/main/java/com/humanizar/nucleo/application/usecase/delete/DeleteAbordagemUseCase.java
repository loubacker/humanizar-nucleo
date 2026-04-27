package com.humanizar.nucleo.application.usecase.delete;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class DeleteAbordagemUseCase {

    private final AbordagemPort abordagemPort;

    public DeleteAbordagemUseCase(AbordagemPort abordagemPort) {
        this.abordagemPort = abordagemPort;
    }

    public void execute(UUID abordagemId) {
        try {
            if (!abordagemPort.existsById(abordagemId)) {
                throw new NucleoException(ReasonCode.ABORDAGEM_NOT_FOUND);
            }
            abordagemPort.deleteById(abordagemId);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (RuntimeException runtimeException) {
            throw new NucleoException(ReasonCode.ABORDAGEM_DELETE_FAILED, runtimeException);
        }
    }
}
