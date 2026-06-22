package com.humanizar.nucleo.domain.exception;

import com.humanizar.nucleo.domain.model.enums.ReasonCode;

public class NucleoException extends RuntimeException {

    private final ReasonCode reasonCode;
    private final String correlationId;

    public NucleoException(ReasonCode reasonCode) {
        this(reasonCode, reasonCode.message(), null, null);
    }

    public NucleoException(ReasonCode reasonCode, Throwable cause) {
        this(reasonCode, reasonCode.message(), null, cause);
    }

    public NucleoException(ReasonCode reasonCode, String message) {
        this(reasonCode, message, null, null);
    }

    public NucleoException(ReasonCode reasonCode, String message, String correlationId) {
        this(reasonCode, message, correlationId, null);
    }

    public NucleoException(ReasonCode reasonCode, String message, Throwable cause) {
        this(reasonCode, message, null, cause);
    }

    public NucleoException(ReasonCode reasonCode, String message, String correlationId, Throwable cause) {
        super(message, cause);
        this.reasonCode = reasonCode;
        this.correlationId = correlationId;
    }

    public ReasonCode reasonCode() {
        return reasonCode;
    }

    public ReasonCode getReasonCode() {
        return reasonCode;
    }

    public String getCorrelationId() {
        return correlationId;
    }

    public boolean isRetryable() {
        return reasonCode.isRetryable();
    }
}
