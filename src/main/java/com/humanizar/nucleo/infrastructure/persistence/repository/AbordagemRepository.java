package com.humanizar.nucleo.infrastructure.persistence.repository;

import com.humanizar.nucleo.infrastructure.persistence.entity.AbordagemEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.List;
import java.util.UUID;

public interface AbordagemRepository extends JpaRepository<AbordagemEntity, UUID> {

    boolean existsByNucleoId(UUID nucleoId);

    List<AbordagemEntity> findByNucleoId(UUID nucleoId);
}
