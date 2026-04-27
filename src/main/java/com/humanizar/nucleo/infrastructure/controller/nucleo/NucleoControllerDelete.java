package com.humanizar.nucleo.infrastructure.controller.nucleo;

import com.humanizar.nucleo.application.service.nucleo.NucleoServiceDelete;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/v1")
public class NucleoControllerDelete {

    private final NucleoServiceDelete nucleoServiceDelete;

    public NucleoControllerDelete(NucleoServiceDelete nucleoServiceDelete) {
        this.nucleoServiceDelete = nucleoServiceDelete;
    }

    @DeleteMapping("/nucleo/delete/{id}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<Void> delete(@PathVariable UUID id) {
        nucleoServiceDelete.execute(id);
        return ResponseEntity.noContent().build();
    }
}
