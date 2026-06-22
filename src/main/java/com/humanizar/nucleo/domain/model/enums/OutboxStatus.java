package com.humanizar.nucleo.domain.model.enums;

public enum OutboxStatus {
    NEW,
    LOCKED,
    PUBLISHED,
    FAILED,
    DEAD
}
