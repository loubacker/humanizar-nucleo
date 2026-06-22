package com.humanizar.nucleo.infrastructure.messaging.inbound.idempotency;

import java.util.UUID;

import org.springframework.stereotype.Component;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.ProcessedEventPort;

@Component
public class ProcessedEventGuard {

    private final ProcessedEventPort processedEventPort;

    public ProcessedEventGuard(ProcessedEventPort processedEventPort) {
        this.processedEventPort = processedEventPort;
    }

    public void ensureNotProcessed(String consumerName, UUID eventId, String correlationId) {
        if (processedEventPort.existsByConsumerNameAndEventId(consumerName, eventId)) {
            throw new NucleoException(ReasonCode.DUPLICATE_EVENT, ReasonCode.DUPLICATE_EVENT.message(), correlationId);
        }
    }
}
