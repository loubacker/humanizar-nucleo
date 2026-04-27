package com.humanizar.nucleoservice.application.service;

import com.humanizar.nucleo.application.mapper.AbordagemRetrieveMapper;
import com.humanizar.nucleo.application.mapper.NucleoCreateMapper;
import com.humanizar.nucleo.application.mapper.NucleoRetrieveMapper;
import com.humanizar.nucleo.application.service.nucleo.NucleoServiceCreate;
import com.humanizar.nucleo.application.usecase.create.CreateNucleoUseCase;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;

class NucleoServiceCreateTest {

    @Test
    void shouldPropagatePayloadRequiredWhenPayloadIsNull() {
        NucleoPort nucleoPort = Mockito.mock(NucleoPort.class);
        CreateNucleoUseCase createNucleoUseCase = new CreateNucleoUseCase(nucleoPort, new NucleoCreateMapper());
        NucleoRetrieveMapper nucleoRetrieveMapper = new NucleoRetrieveMapper(new AbordagemRetrieveMapper());
        NucleoServiceCreate nucleoServiceCreate = new NucleoServiceCreate(createNucleoUseCase, nucleoRetrieveMapper);

        NucleoException nucleoException = Assertions.assertThrows(
                NucleoException.class,
                () -> nucleoServiceCreate.execute(null));

        Assertions.assertEquals(ReasonCode.PAYLOAD_REQUIRED, nucleoException.reasonCode());
    }
}
