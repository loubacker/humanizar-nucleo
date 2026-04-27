package com.humanizar.nucleoservice.application.service;

import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.mapper.AbordagemCreateMapper;
import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.service.abordagem.AbordagemServiceCreate;
import com.humanizar.nucleo.application.usecase.create.CreateAbordagemUseCase;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;

class AbordagemServiceCreateTest {

    @Test
    void shouldPropagateNucleoIdRequiredWhenPayloadHasNoNucleoId() {
        AbordagemPort abordagemPort = Mockito.mock(AbordagemPort.class);
        NucleoPort nucleoPort = Mockito.mock(NucleoPort.class);
        CreateAbordagemUseCase createAbordagemUseCase =
                new CreateAbordagemUseCase(abordagemPort, nucleoPort, new AbordagemCreateMapper());
        AbordagemServiceCreate abordagemServiceCreate =
                new AbordagemServiceCreate(createAbordagemUseCase, new AbordagemRetrieveMapper());

        AbordagemRegisterDTO abordagemRegisterDTO =
                new AbordagemRegisterDTO("ABA", "Descricao", null);

        NucleoException nucleoException = Assertions.assertThrows(
                NucleoException.class,
                () -> abordagemServiceCreate.execute(abordagemRegisterDTO));

        Assertions.assertEquals(ReasonCode.NUCLEO_ID_REQUIRED, nucleoException.reasonCode());
    }
}
