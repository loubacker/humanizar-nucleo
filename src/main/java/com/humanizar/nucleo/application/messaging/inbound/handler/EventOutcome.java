package com.humanizar.nucleo.application.messaging.inbound.handler;

import com.humanizar.nucleo.domain.model.enums.ProcessedResult;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;

public record EventOutcome(
        ProcessedResult result,
        ReasonCode reasonCode,
        String errorMessage,
        boolean retryable) {

    public static EventOutcome success() {
        return new EventOutcome(ProcessedResult.SUCCESS, null, null, false);
    }

    public static EventOutcome failed(ReasonCode reasonCode, String detail) {
        return new EventOutcome(ProcessedResult.FAILED, reasonCode, detail, reasonCode.isRetryable());
    }
}
