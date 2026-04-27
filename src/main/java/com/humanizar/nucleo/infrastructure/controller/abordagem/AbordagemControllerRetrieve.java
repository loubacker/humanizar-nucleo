package com.humanizar.nucleo.infrastructure.controller.abordagem;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.service.abordagem.AbordagemServiceRetrieve;
import com.humanizar.nucleo.infrastructure.config.ResilientMethodsConfig.Retry;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import java.util.UUID;

@RestController
@RequestMapping("/api/v1")
public class AbordagemControllerRetrieve {

    private final AbordagemServiceRetrieve abordagemServiceRetrieve;

    public AbordagemControllerRetrieve(AbordagemServiceRetrieve abordagemServiceRetrieve) {
        this.abordagemServiceRetrieve = abordagemServiceRetrieve;
    }

    @Retry
    @GetMapping("/abordagem/{id}")
    public AbordagemRetrieveDTO findById(@PathVariable UUID id) {
        return abordagemServiceRetrieve.execute(id);
    }
}
