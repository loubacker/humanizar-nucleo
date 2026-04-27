package com.humanizar.nucleoservice.infrastructure.adapter;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.infrastructure.adapter.NucleoAdapter;
import com.humanizar.nucleo.infrastructure.persistence.repository.AbordagemRepository;
import com.humanizar.nucleo.infrastructure.persistence.repository.NucleoRepository;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.mockito.Mockito;

import java.util.UUID;

class NucleoAdapterTest {

    @Test
    void shouldBlockDeleteWhenNucleoHasAbordagens() {
        NucleoRepository nucleoRepository = Mockito.mock(NucleoRepository.class);
        AbordagemRepository abordagemRepository = Mockito.mock(AbordagemRepository.class);
        NucleoAdapter nucleoAdapter = new NucleoAdapter(nucleoRepository, abordagemRepository);
        UUID nucleoId = UUID.randomUUID();

        Mockito.when(abordagemRepository.existsByNucleoId(nucleoId)).thenReturn(true);

        NucleoException nucleoException = Assertions.assertThrows(
                NucleoException.class,
                () -> nucleoAdapter.deleteById(nucleoId));

        Assertions.assertEquals(ReasonCode.NUCLEO_HAS_ABORDAGENS, nucleoException.reasonCode());
        Mockito.verify(nucleoRepository, Mockito.never()).deleteById(Mockito.any(UUID.class));
    }
}
