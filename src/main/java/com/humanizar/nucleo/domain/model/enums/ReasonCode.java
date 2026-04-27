package com.humanizar.nucleo.domain.model.enums;

public enum ReasonCode {
    PAYLOAD_REQUIRED(400, "Payload obrigatorio"),
    NOME_REQUIRED(400, "Nome obrigatorio"),
    DESCRICAO_REQUIRED(400, "Descricao obrigatoria"),
    NUCLEO_ID_REQUIRED(400, "Nucleo id obrigatorio"),
    REQUEST_BODY_INVALID(400, "Corpo da requisicao invalido"),
    REQUEST_PARAMETER_MISSING(400, "Parametro da requisicao ausente"),
    REQUEST_PARAMETER_INVALID(400, "Parametro da requisicao invalido"),
    AUTHENTICATION_REQUIRED(401, "Autenticacao obrigatoria"),
    AUTHORIZATION_DENIED(403, "Usuario nao autorizado"),
    NUCLEO_NOT_FOUND(404, "Nucleo nao encontrado"),
    ABORDAGEM_NOT_FOUND(404, "Abordagem nao encontrada"),
    NUCLEO_HAS_ABORDAGENS(409, "Nucleo possui abordagens vinculadas"),
    NUCLEO_CREATE_FAILED(409, "Falha ao criar nucleo"),
    NUCLEO_UPDATE_FAILED(409, "Falha ao atualizar nucleo"),
    NUCLEO_DELETE_FAILED(409, "Falha ao excluir nucleo"),
    ABORDAGEM_CREATE_FAILED(409, "Falha ao criar abordagem"),
    ABORDAGEM_UPDATE_FAILED(409, "Falha ao atualizar abordagem"),
    ABORDAGEM_DELETE_FAILED(409, "Falha ao excluir abordagem"),
    DATABASE_ACCESS_FAILED(500, "Falha de acesso ao banco de dados"),
    UNEXPECTED_ERROR(500, "Erro inesperado");

    private final int httpStatus;
    private final String message;

    ReasonCode(int httpStatus, String message) {
        this.httpStatus = httpStatus;
        this.message = message;
    }

    public int httpStatus() {
        return httpStatus;
    }

    public String message() {
        return message;
    }
}
