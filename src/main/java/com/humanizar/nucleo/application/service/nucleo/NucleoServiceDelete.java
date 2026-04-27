package com.humanizar.nucleo.application.service.nucleo;

import com.humanizar.nucleo.application.usecase.delete.DeleteNucleoUseCase;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.UUID;

@Service
@Transactional
public class NucleoServiceDelete {

    private final DeleteNucleoUseCase deleteNucleoUseCase;

    public NucleoServiceDelete(DeleteNucleoUseCase deleteNucleoUseCase) {
        this.deleteNucleoUseCase = deleteNucleoUseCase;
    }

    public void execute(UUID nucleoId) {
        deleteNucleoUseCase.execute(nucleoId);
    }
}
