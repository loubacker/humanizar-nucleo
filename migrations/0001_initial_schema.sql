BEGIN;

CREATE TABLE nucleo (
    id UUID PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    descricao VARCHAR(255) NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE abordagem (
    id UUID PRIMARY KEY,
    nome VARCHAR(255) NOT NULL,
    descricao VARCHAR(255) NOT NULL,
    nucleo_id UUID NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    CONSTRAINT fk_abordagem_nucleo
        FOREIGN KEY (nucleo_id)
        REFERENCES nucleo (id)
        ON DELETE RESTRICT
);

CREATE INDEX idx_abordagem_nucleo ON abordagem (nucleo_id, nome, id);

CREATE TABLE outbox_event (
    id BIGSERIAL PRIMARY KEY,
    event_id UUID NOT NULL,
    correlation_id UUID NOT NULL,
    producer_service VARCHAR(255) NOT NULL,
    exchange_name VARCHAR(255) NOT NULL,
    routing_key VARCHAR(255) NOT NULL,
    aggregate_type VARCHAR(255) NOT NULL,
    aggregate_id UUID NOT NULL,
    event_version SMALLINT NOT NULL,
    payload TEXT NOT NULL,
    actor_id UUID,
    user_agent VARCHAR(255),
    origin_ip VARCHAR(255),
    status VARCHAR(255) NOT NULL,
    attempt_count INTEGER NOT NULL,
    max_attempts INTEGER NOT NULL,
    next_retry_at TIMESTAMP,
    last_error TEXT,
    created_at TIMESTAMP NOT NULL,
    published_at TIMESTAMP,
    locked_by UUID,
    CONSTRAINT uk_outbox_event_id UNIQUE (event_id),
    CONSTRAINT ck_outbox_event_version CHECK (event_version > 0),
    CONSTRAINT ck_outbox_status CHECK (status IN ('NEW', 'LOCKED', 'PUBLISHED', 'FAILED', 'DEAD')),
    CONSTRAINT ck_outbox_attempt_count CHECK (attempt_count >= 0),
    CONSTRAINT ck_outbox_max_attempts CHECK (max_attempts > 0)
);

CREATE INDEX idx_outbox_dispatch ON outbox_event (status, next_retry_at, created_at, id);
CREATE INDEX idx_outbox_correlation ON outbox_event (correlation_id);
CREATE INDEX idx_outbox_aggregate ON outbox_event (aggregate_type, aggregate_id, created_at);

CREATE TABLE processed_event (
    id BIGSERIAL PRIMARY KEY,
    consumer_name VARCHAR(255) NOT NULL,
    event_id UUID NOT NULL,
    event_hash CHAR(64),
    response_event_id UUID,
    correlation_id UUID NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    aggregate_type VARCHAR(255) NOT NULL,
    aggregate_id UUID NOT NULL,
    actor_id UUID,
    user_agent VARCHAR(255),
    origin_ip VARCHAR(255),
    processed_at TIMESTAMP NOT NULL,
    result VARCHAR(255) NOT NULL,
    reason_code VARCHAR(255),
    error_message TEXT,
    CONSTRAINT uk_processed_consumer_event UNIQUE (consumer_name, event_id),
    CONSTRAINT ck_processed_result CHECK (result IN ('SUCCESS', 'IGNORED', 'FAILED'))
);

CREATE INDEX idx_processed_correlation ON processed_event (correlation_id);
CREATE INDEX idx_processed_event ON processed_event (event_id);
CREATE INDEX idx_processed_time ON processed_event (processed_at, id);
CREATE INDEX idx_processed_reason_code ON processed_event (reason_code);

COMMIT;
