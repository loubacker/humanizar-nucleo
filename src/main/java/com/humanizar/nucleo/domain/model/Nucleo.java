package com.humanizar.nucleo.domain.model;

import java.time.LocalDateTime;
import java.util.Objects;
import java.util.UUID;

public class Nucleo {

    private UUID id;
    private String nome;
    private String descricao;
    private LocalDateTime createdAt;
    private LocalDateTime updatedAt;

    public Nucleo() {
    }

    public Nucleo(UUID id, String nome, String descricao, LocalDateTime createdAt, LocalDateTime updatedAt) {
        this.id = id;
        this.nome = nome;
        this.descricao = descricao;
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
        if (!(other instanceof Nucleo nucleo)) {
            return false;
        }
        return Objects.equals(id, nucleo.id)
                && Objects.equals(nome, nucleo.nome)
                && Objects.equals(descricao, nucleo.descricao)
                && Objects.equals(createdAt, nucleo.createdAt)
                && Objects.equals(updatedAt, nucleo.updatedAt);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, nome, descricao, createdAt, updatedAt);
    }

    @Override
    public String toString() {
        return "Nucleo{" +
                "id=" + id +
                ", nome='" + nome + '\'' +
                ", descricao='" + descricao + '\'' +
                ", createdAt=" + createdAt +
                ", updatedAt=" + updatedAt +
                '}';
    }

    public static class Builder {

        private UUID id;
        private String nome;
        private String descricao;
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

        public Builder createdAt(LocalDateTime createdAt) {
            this.createdAt = createdAt;
            return this;
        }

        public Builder updatedAt(LocalDateTime updatedAt) {
            this.updatedAt = updatedAt;
            return this;
        }

        public Nucleo build() {
            return new Nucleo(id, nome, descricao, createdAt, updatedAt);
        }
    }
}
