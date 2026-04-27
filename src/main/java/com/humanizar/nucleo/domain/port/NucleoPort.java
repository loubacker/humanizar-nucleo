package com.humanizar.nucleo.domain.port;

import com.humanizar.nucleo.domain.model.Nucleo;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

public interface NucleoPort {

    Nucleo save(Nucleo nucleo);

    Optional<Nucleo> findById(UUID id);

    List<Nucleo> findAll();

    void deleteById(UUID id);

    boolean existsById(UUID id);
}
