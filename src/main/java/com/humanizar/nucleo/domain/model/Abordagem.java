package com.humanizar.nucleo.domain.model;

import java.time.LocalDateTime;
import java.util.Objects;
import java.util.UUID;

public class Abordagem {

    private UUID id;
    private String nome;
    private String descricao;
    private UUID nucleoId;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    public Abordagem() {
    }

    public Abordagem(UUID id,
                     String nome,
                     String descricao,
                     UUID nucleoId,
                     LocalDateTime createdAt,
                     LocalDateTime updatedAt) {
        this.id = id;
        this.nome = nome;
        this.descricao = descricao;
        this.nucleoId = nucleoId;
        this.createdAt = createdAt;
        this.updatedAt = updatedAt;
    }

    public static Builder builder() {
        return new Builder();
    }

    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getNome() {
        return nome;
    }

    public void setNome(String nome) {
        this.nome = nome;
    }

    public String getDescricao() {
        return descricao;
    }

    public void setDescricao(String descricao) {
        this.descricao = descricao;
    }

    public UUID getNucleoId() {
        return nucleoId;
    }

    public void setNucleoId(UUID nucleoId) {
        this.nucleoId = nucleoId;
    }

    public LocalDateTime getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(LocalDateTime createdAt) {
        this.createdAt = createdAt;
    }

    public LocalDateTime getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(LocalDateTime updatedAt) {
        this.updatedAt = updatedAt;
    }

    @Override
    public boolean equals(Object other) {
        if (this == other) {
            return true;
        }
        if (!(other instanceof Abordagem abordagem)) {
            return false;
        }
        return Objects.equals(id, abordagem.id)
                && Objects.equals(nome, abordagem.nome)
                && Objects.equals(descricao, abordagem.descricao)
                && Objects.equals(nucleoId, abordagem.nucleoId)
                && Objects.equals(createdAt, abordagem.createdAt)
                && Objects.equals(updatedAt, abordagem.updatedAt);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, nome, descricao, nucleoId, createdAt, updatedAt);
    }

    @Override
    public String toString() {
        return "Abordagem{" +
                "id=" + id +
                ", nome='" + nome + '\'' +
                ", descricao='" + descricao + '\'' +
                ", nucleoId=" + nucleoId +
                ", createdAt=" + createdAt +
                ", updatedAt=" + updatedAt +
                '}';
    }

    public static class Builder {

        private UUID id;
        private String nome;
        private String descricao;
        private UUID nucleoId;
        private LocalDateTime createdAt;
        private LocalDateTime updatedAt;

        public Builder id(UUID id) {
            this.id = id;
            return this;
        }

        public Builder nome(String nome) {
            this.nome = nome;
            return this;
        }

        public Builder descricao(String descricao) {
            this.descricao = descricao;
            return this;
        }

        public Builder nucleoId(UUID nucleoId) {
            this.nucleoId = nucleoId;
            return this;
        }

        public Builder createdAt(LocalDateTime createdAt) {
            this.createdAt = createdAt;
            return this;
        }

        public Builder updatedAt(LocalDateTime updatedAt) {
            this.updatedAt = updatedAt;
            return this;
        }

        public Abordagem build() {
            return new Abordagem(id, nome, descricao, nucleoId, createdAt, updatedAt);
        }
    }
}
