package com.humanizar.nucleo.application.service.abordagem;

import com.humanizar.nucleo.application.usecase.delete.DeleteAbordagemUseCase;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@Transactional
public class AbordagemServiceDelete {

    private final DeleteAbordagemUseCase deleteAbordagemUseCase;

    public AbordagemServiceDelete(DeleteAbordagemUseCase deleteAbordagemUseCase) {
        this.deleteAbordagemUseCase = deleteAbordagemUseCase;
    }

    public void execute(UUID abordagemId) {
        deleteAbordagemUseCase.execute(abordagemId);
    }
}
