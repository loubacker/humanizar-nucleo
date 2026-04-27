package com.humanizar.nucleo.infrastructure.controller.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.application.service.nucleo.NucleoServiceUpdate;
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
public class NucleoControllerUpdate {

    private final NucleoServiceUpdate nucleoServiceUpdate;

    public NucleoControllerUpdate(NucleoServiceUpdate nucleoServiceUpdate) {
        this.nucleoServiceUpdate = nucleoServiceUpdate;
    }

    @PutMapping("/nucleo/update/{id}")
    @PreAuthorize("hasRole('ADMINISTRADOR')")
    public ResponseEntity<NucleoRetrieveDTO> update(@PathVariable UUID id, @RequestBody NucleoUpdateDTO nucleoUpdateDTO) {
        NucleoRetrieveDTO response = nucleoServiceUpdate.execute(id, nucleoUpdateDTO);
        return ResponseEntity.ok(response);
    }
}
