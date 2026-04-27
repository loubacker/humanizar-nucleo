package com.humanizar.nucleo.infrastructure.controller.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.application.service.abordagem.AbordagemServiceUpdate;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.PutMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/v1")
public class AbordagemControllerUpdate {

    private final AbordagemServiceUpdate abordagemServiceUpdate;

    public AbordagemControllerUpdate(AbordagemServiceUpdate abordagemServiceUpdate) {
        this.abordagemServiceUpdate = abordagemServiceUpdate;
    }

    @PutMapping("/abordagem/update/{id}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<AbordagemRetrieveDTO> update(@PathVariable UUID id,
                                                       @RequestBody AbordagemUpdateDTO abordagemUpdateDTO) {
        AbordagemRetrieveDTO response = abordagemServiceUpdate.execute(id, abordagemUpdateDTO);
        return ResponseEntity.ok(response);
    }
}
