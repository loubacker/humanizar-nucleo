package com.humanizar.nucleo.domain.model.enums;

public enum ReasonCode {

    // ============================================================
    //  400 — Bad Request
    // ============================================================

    // -- Requisicao generica
    PAYLOAD_REQUIRED(400, "Payload obrigatorio", false),
    REQUEST_BODY_INVALID(400, "Corpo da requisicao invalido", false),
    REQUEST_PARAMETER_MISSING(400, "Parametro da requisicao ausente", false),
    REQUEST_PARAMETER_INVALID(400, "Parametro da requisicao invalido", false),
    VALIDATION_ERROR(400, "Falha de validacao do payload/evento.", false),

    // -- Nucleo e abordagem (dados basicos)
    NOME_REQUIRED(400, "Nome obrigatorio", false),
    DESCRICAO_REQUIRED(400, "Descricao obrigatoria", false),
    NUCLEO_ID_REQUIRED(400, "Nucleo id obrigatorio", false),

    // -- Mensageria inbound
    INBOUND_REQUIRED_FIELD(400, "Campo obrigatorio ausente no inbound.", false),
    INBOUND_PARSE_ERROR(400, "Falha ao parsear mensagem inbound.", false),
    UNSUPPORTED_ROUTING_KEY(400, "Routing key nao suportada.", false),

    // ============================================================
    //  401 — Unauthorized
    // ============================================================

    AUTHENTICATION_REQUIRED(401, "Autenticacao obrigatoria", false),

    // ============================================================
    //  403 — Forbidden
    // ============================================================

    AUTHORIZATION_DENIED(403, "Usuario nao autorizado", false),

    // ============================================================
    //  404 — Not Found
    // ============================================================

    NUCLEO_NOT_FOUND(404, "Nucleo nao encontrado", false),
    ABORDAGEM_NOT_FOUND(404, "Abordagem nao encontrada", false),

    // ============================================================
    //  409 — Conflict
    // ============================================================

    NUCLEO_HAS_ABORDAGENS(409, "Nucleo possui abordagens vinculadas", false),
    NUCLEO_CREATE_FAILED(409, "Falha ao criar nucleo", false),
    NUCLEO_UPDATE_FAILED(409, "Falha ao atualizar nucleo", false),
    NUCLEO_DELETE_FAILED(409, "Falha ao excluir nucleo", false),
    ABORDAGEM_CREATE_FAILED(409, "Falha ao criar abordagem", false),
    ABORDAGEM_UPDATE_FAILED(409, "Falha ao atualizar abordagem", false),
    ABORDAGEM_DELETE_FAILED(409, "Falha ao excluir abordagem", false),

    // -- Idempotencia e eventos
    DUPLICATE_EVENT(409, "Evento duplicado ja processado.", false),

    // ============================================================
    //  422 — Unprocessable Entity
    // ============================================================

    UNSUPPORTED_EVENT_VERSION(422, "Versao de evento nao suportada.", false),

    // ============================================================
    //  500 — Internal Server Error
    // ============================================================

    DATABASE_ACCESS_FAILED(500, "Falha de acesso ao banco de dados", true),
    UNEXPECTED_ERROR(500, "Erro inesperado", false),

    // ============================================================
    //  502 — Bad Gateway (integracoes externas)
    // ============================================================

    INTEGRATION_FAILURE(502, "Falha de integracao com dependencia externa.", true),

    // ============================================================
    //  503 — Service Unavailable
    // ============================================================

    PERSISTENCE_FAILURE(503, "Falha de persistencia no banco de dados.", true);

    private final int httpStatus;
    private final String message;
    private final boolean retryable;

    ReasonCode(int httpStatus, String message, boolean retryable) {
        this.httpStatus = httpStatus;
        this.message = message;
        this.retryable = retryable;
    }

    public int httpStatus() {
        return httpStatus;
    }

    public String message() {
        return message;
    }

    public int getStatusCode() {
        return httpStatus;
    }

    public String getMessage() {
        return message;
    }

    public boolean isRetryable() {
        return retryable;
    }
}
