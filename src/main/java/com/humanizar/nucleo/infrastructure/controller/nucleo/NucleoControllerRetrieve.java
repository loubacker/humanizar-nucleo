package com.humanizar.nucleo.infrastructure.controller.nucleo;

import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.service.nucleo.NucleoServiceRetrieve;
import com.humanizar.nucleo.infrastructure.config.ResilientMethodsConfig.Retry;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/v1")
public class NucleoControllerRetrieve {

    private final NucleoServiceRetrieve nucleoServiceRetrieve;

    public NucleoControllerRetrieve(NucleoServiceRetrieve nucleoServiceRetrieve) {
        this.nucleoServiceRetrieve = nucleoServiceRetrieve;
    }

    @Retry
    @GetMapping("/nucleos")
    public List<NucleoRetrieveDTO> findAll() {
        return nucleoServiceRetrieve.executeWithoutAbordagens();
    }

    @Retry
    @GetMapping("/nucleos/abordagens")
    public List<NucleoRetrieveDTO> findAllWithAbordagens() {
        return nucleoServiceRetrieve.executeWithAbordagens();
    }

    @Retry
    @GetMapping("/nucleo/{id}")
    public NucleoRetrieveDTO findById(@PathVariable UUID id) {
        return nucleoServiceRetrieve.executeById(id);
    }
}
