package com.humanizar.nucleo.infrastructure.controller.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.service.nucleo.NucleoServiceCreate;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/api/v1")
public class NucleoControllerCreate {

    private final NucleoServiceCreate nucleoServiceCreate;

    public NucleoControllerCreate(NucleoServiceCreate nucleoServiceCreate) {
        this.nucleoServiceCreate = nucleoServiceCreate;
    }

    @PostMapping("/nucleo/register")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<NucleoRetrieveDTO> create(@RequestBody NucleoRegisterDTO nucleoRegisterDTO) {
        NucleoRetrieveDTO response = nucleoServiceCreate.execute(nucleoRegisterDTO);
        return ResponseEntity.status(HttpStatus.CREATED).body(response);
    }
}
