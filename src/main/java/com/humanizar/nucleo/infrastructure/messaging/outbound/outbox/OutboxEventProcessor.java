package com.humanizar.nucleo.infrastructure.messaging.outbound.outbox;

import java.time.Duration;
import java.time.LocalDateTime;
import java.util.List;
import java.util.Objects;
import java.util.UUID;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.nucleo.domain.model.OutboxEvent;
import com.humanizar.nucleo.domain.model.enums.OutboxStatus;
import com.humanizar.nucleo.domain.port.OutboxEventPort;
import com.humanizar.nucleo.infrastructure.messaging.outbound.rabbit.RabbitOutboxPublisher;

@Component
public class OutboxEventProcessor {

    private static final Logger log = LoggerFactory.getLogger(OutboxEventProcessor.class);
    private static final Duration LOCK_TIMEOUT = Duration.ofMinutes(5);

    private final UUID instanceId = UUID.randomUUID();
    private final OutboxEventPort outboxEventPort;
    private final RabbitOutboxPublisher rabbitOutboxPublisher;
    private final OutboxRetryPolicy retryPolicy;

    public OutboxEventProcessor(
            OutboxEventPort outboxEventPort,
            RabbitOutboxPublisher rabbitOutboxPublisher,
            OutboxRetryPolicy retryPolicy) {
        this.outboxEventPort = outboxEventPort;
        this.rabbitOutboxPublisher = rabbitOutboxPublisher;
        this.retryPolicy = retryPolicy;
        log.info("OutboxEventProcessor iniciado. instanceId={}", instanceId);
    }

    @Transactional
    public List<OutboxEvent> claimBatch(int batchSize) {
        List<OutboxEvent> pending = outboxEventPort.findPendingForRelay(
                List.of(OutboxStatus.NEW, OutboxStatus.FAILED, OutboxStatus.LOCKED),
                LocalDateTime.now(), batchSize);

        LocalDateTime lockExpiry = LocalDateTime.now().plus(LOCK_TIMEOUT);
        for (OutboxEvent event : pending) {
            event.setStatus(OutboxStatus.LOCKED);
            event.setNextRetryAt(lockExpiry);
            event.setLockedBy(instanceId);
            outboxEventPort.save(event);
        }
        return pending;
    }

    @Transactional
    public void processEvent(OutboxEvent event) {
        OutboxEvent fresh = outboxEventPort.findByEventId(event.getEventId()).orElse(null);
        if (fresh == null
                || fresh.getStatus() != OutboxStatus.LOCKED
                || !instanceId.equals(fresh.getLockedBy())) {
            log.warn("Fencing check falhou — evento re-claimed por outro worker. eventId={}, "
                    + "expectedOwner={}, actualOwner={}",
                    event.getEventId(), instanceId,
                    fresh != null ? fresh.getLockedBy() : "N/A");
            return;
        }

        OutboxEvent processingEvent = fresh;

        try {
            rabbitOutboxPublisher.publish(processingEvent);

            processingEvent.setStatus(OutboxStatus.PUBLISHED);
            processingEvent.setPublishedAt(LocalDateTime.now());
            processingEvent.setLastError(null);
            processingEvent.setLockedBy(null);
            outboxEventPort.save(processingEvent);

        } catch (Exception ex) {
            int newAttemptCount = Objects.requireNonNullElse(processingEvent.getAttemptCount(), 0) + 1;
            processingEvent.setAttemptCount(newAttemptCount);
            processingEvent.setLastError(ex.getClass().getSimpleName() + ": " + ex.getMessage());

            int maxAttempts = Objects.requireNonNullElse(
                    processingEvent.getMaxAttempts(), retryPolicy.getDefaultMaxAttempts());

            if (retryPolicy.isExhausted(newAttemptCount, maxAttempts)) {
                processingEvent.setStatus(OutboxStatus.DEAD);
                log.error("Evento movido para DEAD apos {} tentativas. eventId={}",
                        newAttemptCount, processingEvent.getEventId(), ex);
            } else {
                processingEvent.setStatus(OutboxStatus.FAILED);
                processingEvent.setNextRetryAt(retryPolicy.nextRetryAt(newAttemptCount));
                log.warn("Falha ao publicar evento. attempt={}/{}, nextRetry={}, eventId={}",
                        newAttemptCount, maxAttempts, processingEvent.getNextRetryAt(),
                        processingEvent.getEventId(), ex);
            }

            processingEvent.setLockedBy(null);
            outboxEventPort.save(processingEvent);
        }
    }
}
