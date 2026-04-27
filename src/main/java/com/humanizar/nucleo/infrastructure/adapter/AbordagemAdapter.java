package com.humanizar.nucleo.infrastructure.adapter;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.Abordagem;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.AbordagemPort;
import com.humanizar.nucleo.infrastructure.persistence.entity.AbordagemEntity;
import com.humanizar.nucleo.infrastructure.persistence.repository.AbordagemRepository;
import org.springframework.dao.DataAccessException;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class AbordagemAdapter implements AbordagemPort {

    private final AbordagemRepository abordagemRepository;

    public AbordagemAdapter(AbordagemRepository abordagemRepository) {
        this.abordagemRepository = abordagemRepository;
    }

    @Override
    public Abordagem save(Abordagem abordagem) {
        try {
            AbordagemEntity savedEntity = abordagemRepository.save(toEntity(abordagem));
            return toDomain(savedEntity);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public Optional<Abordagem> findById(UUID id) {
        try {
            return abordagemRepository.findById(id).map(this::toDomain);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public List<Abordagem> findByNucleoId(UUID nucleoId) {
        try {
            return abordagemRepository.findByNucleoId(nucleoId).stream()
                    .map(this::toDomain)
                    .toList();
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public void deleteById(UUID id) {
        try {
            abordagemRepository.deleteById(id);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    @Override
    public boolean existsById(UUID id) {
        try {
            return abordagemRepository.existsById(id);
        } catch (DataAccessException dataAccessException) {
            throw new NucleoException(ReasonCode.DATABASE_ACCESS_FAILED, dataAccessException);
        }
    }

    private AbordagemEntity toEntity(Abordagem abordagem) {
        AbordagemEntity entity = new AbordagemEntity();
        entity.setId(abordagem.getId());
        entity.setNome(abordagem.getNome());
        entity.setDescricao(abordagem.getDescricao());
        entity.setNucleoId(abordagem.getNucleoId());
        entity.setCreatedAt(abordagem.getCreatedAt());
        entity.setUpdatedAt(abordagem.getUpdatedAt());
        return entity;
    }

    private Abordagem toDomain(AbordagemEntity entity) {
        return Abordagem.builder()
                .id(entity.getId())
                .nome(entity.getNome())
                .descricao(entity.getDescricao())
                .nucleoId(entity.getNucleoId())
                .createdAt(entity.getCreatedAt())
                .updatedAt(entity.getUpdatedAt())
                .build();
    }
}
