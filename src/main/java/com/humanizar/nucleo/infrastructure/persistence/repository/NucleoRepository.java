package com.humanizar.nucleo.infrastructure.persistence.repository;

import com.humanizar.nucleo.infrastructure.persistence.entity.NucleoEntity;
import org.springframework.data.jpa.repository.JpaRepository;

import java.util.UUID;

public interface NucleoRepository extends JpaRepository<NucleoEntity, UUID> {
}
