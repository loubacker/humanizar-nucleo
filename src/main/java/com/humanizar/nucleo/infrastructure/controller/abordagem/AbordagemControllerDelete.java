package com.humanizar.nucleo.infrastructure.controller.abordagem;

import com.humanizar.nucleo.application.service.abordagem.AbordagemServiceDelete;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/v1")
public class AbordagemControllerDelete {

    private final AbordagemServiceDelete abordagemServiceDelete;

    public AbordagemControllerDelete(AbordagemServiceDelete abordagemServiceDelete) {
        this.abordagemServiceDelete = abordagemServiceDelete;
    }

    @DeleteMapping("/abordagem/delete/{id}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<Void> delete(@PathVariable UUID id) {
        abordagemServiceDelete.execute(id);
        return ResponseEntity.noContent().build();
    }
}
