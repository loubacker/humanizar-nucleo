package com.humanizar.nucleo.domain.port;

import com.humanizar.nucleo.domain.model.Abordagem;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface AbordagemPort {

    Abordagem save(Abordagem abordagem);

    Optional<Abordagem> findById(UUID id);

    List<Abordagem> findByNucleoId(UUID nucleoId);

    void deleteById(UUID id);

    boolean existsById(UUID id);
}
