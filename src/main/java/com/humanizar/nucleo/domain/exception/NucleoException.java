package com.humanizar.nucleo.domain.exception;

import com.humanizar.nucleo.domain.model.enums.ReasonCode;

public class NucleoException extends RuntimeException {

    private final ReasonCode reasonCode;

    public NucleoException(ReasonCode reasonCode) {
        super(reasonCode.message());
        this.reasonCode = reasonCode;
    }

    public NucleoException(ReasonCode reasonCode, Throwable cause) {
        super(reasonCode.message(), cause);
        this.reasonCode = reasonCode;
    }

    public ReasonCode reasonCode() {
        return reasonCode;
    }
}
