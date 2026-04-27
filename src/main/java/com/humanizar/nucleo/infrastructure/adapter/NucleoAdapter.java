package com.humanizar.nucleo.infrastructure.adapter;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;
import com.humanizar.nucleo.infrastructure.persistence.entity.NucleoEntity;
import com.humanizar.nucleo.infrastructure.persistence.repository.AbordagemRepository;
import com.humanizar.nucleo.infrastructure.persistence.repository.NucleoRepository;
import org.springframework.dao.DataAccessException;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class NucleoAdapter implements NucleoPort {

    private final NucleoRepository nucleoRepository;
    private final AbordagemRepository abordagemRepository;

    public NucleoAdapter(NucleoRepository nucleoRepository, AbordagemRepository abordagemRepository) {
        this.nucleoRepository = nucleoRepository;
        this.abordagemRepository = abordagemRepository;
    }

    @Override
    public Nucleo save(Nucleo nucleo) {
        try {
            NucleoEntity savedEntity = nucleoRepository.save(toEntity(nucleo));
            return toDomain(savedEntity);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public Optional<Nucleo> findById(UUID id) {
        try {
            return nucleoRepository.findById(id).map(this::toDomain);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public List<Nucleo> findAll() {
        try {
            return nucleoRepository.findAll().stream()
                    .map(this::toDomain)
                    .toList();
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    @Transactional
    public void deleteById(UUID nucleoId) {
        try {
            if (abordagemRepository.existsByNucleoId(nucleoId)) {
                throw new NucleoException(ReasonCode.NUCLEO_HAS_ABORDAGENS);
            }
            if (!nucleoRepository.existsById(nucleoId)) {
                throw new NucleoException(ReasonCode.NUCLEO_NOT_FOUND);
            }
            nucleoRepository.deleteById(nucleoId);
        } catch (NucleoException nucleoException) {
            throw nucleoException;
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public boolean existsById(UUID id) {
        try {
            return nucleoRepository.existsById(id);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    private NucleoEntity toEntity(Nucleo nucleo) {
        NucleoEntity entity = new NucleoEntity();
        entity.setId(nucleo.getId());
        entity.setNome(nucleo.getNome());
        entity.setDescricao(nucleo.getDescricao());
        entity.setCreatedAt(nucleo.getCreatedAt());
        entity.setUpdatedAt(nucleo.getUpdatedAt());
        return entity;
    }

    private Nucleo toDomain(NucleoEntity entity) {
        return Nucleo.builder()
                .id(entity.getId())
                .nome(entity.getNome())
                .descricao(entity.getDescricao())
                .createdAt(entity.getCreatedAt())
                .updatedAt(entity.getUpdatedAt())
                .build();
    }
}
