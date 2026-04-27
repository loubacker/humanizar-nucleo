package com.humanizar.nucleo.infrastructure.controller.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.service.abordagem.AbordagemServiceCreate;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1")
public class AbordagemControllerCreate {

    private final AbordagemServiceCreate abordagemServiceCreate;

    public AbordagemControllerCreate(AbordagemServiceCreate abordagemServiceCreate) {
        this.abordagemServiceCreate = abordagemServiceCreate;
    }

    @PostMapping("/abordagem/register")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<AbordagemRetrieveDTO> create(@RequestBody AbordagemRegisterDTO abordagemRegisterDTO) {
        AbordagemRetrieveDTO response = abordagemServiceCreate.execute(abordagemRegisterDTO);
        return ResponseEntity.status(HttpStatus.CREATED).body(response);
    }
}
