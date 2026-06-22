package com.humanizar.nucleo.infrastructure.messaging.outbound.outbox;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.ArgumentMatchers.anyInt;
import static org.mockito.ArgumentMatchers.anyList;
import static org.mockito.Mockito.doThrow;
import static org.mockito.Mockito.never;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

import java.time.LocalDateTime;
import java.util.List;
import java.util.Optional;
import java.util.UUID;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import com.humanizar.nucleo.domain.model.OutboxEvent;
import com.humanizar.nucleo.domain.model.enums.OutboxStatus;
import com.humanizar.nucleo.domain.port.OutboxEventPort;
import com.humanizar.nucleo.infrastructure.messaging.outbound.rabbit.RabbitOutboxPublisher;

@ExtendWith(MockitoExtension.class)
class OutboxEventProcessorTest {

    @Mock
    private OutboxEventPort outboxEventPort;

    @Mock
    private RabbitOutboxPublisher rabbitOutboxPublisher;

    @Mock
    private OutboxRetryPolicy outboxRetryPolicy;

    private OutboxEventProcessor outboxEventProcessor;

    @BeforeEach
    void setUp() {
        outboxEventProcessor = new OutboxEventProcessor(
                outboxEventPort,
                rabbitOutboxPublisher,
                outboxRetryPolicy);
    }

    @Test
    void shouldClaimAndLockEventsFromOutbox() {
        OutboxEvent event = newOutboxEvent(OutboxStatus.NEW);
        when(outboxEventPort.findPendingForRelay(anyList(), any(LocalDateTime.class), anyInt()))
                .thenReturn(List.of(event));
        when(outboxEventPort.save(any(OutboxEvent.class))).thenAnswer(invocation -> invocation.getArgument(0));

        List<OutboxEvent> claimed = outboxEventProcessor.claimBatch(20);

        assertEquals(1, claimed.size());
        assertEquals(OutboxStatus.LOCKED, event.getStatus());
        assertNotNull(event.getLockedBy());
        assertNotNull(event.getNextRetryAt());
        verify(outboxEventPort).save(event);
    }

    @Test
    void shouldPublishAndMarkAsPublishedWhenPublisherSucceeds() {
        OutboxEvent event = claimOneEvent();
        when(outboxEventPort.findByEventId(event.getEventId())).thenReturn(Optional.of(event));

        outboxEventProcessor.processEvent(event);

        verify(rabbitOutboxPublisher).publish(event);
        assertEquals(OutboxStatus.PUBLISHED, event.getStatus());
        assertNotNull(event.getPublishedAt());
        assertEquals(null, event.getLockedBy());
    }

    @Test
    void shouldSkipProcessingWhenFencingFails() {
        OutboxEvent staleEvent = newOutboxEvent(OutboxStatus.NEW);
        OutboxEvent freshEvent = newOutboxEventWithEventId(staleEvent.getEventId(), OutboxStatus.NEW);
        when(outboxEventPort.findByEventId(staleEvent.getEventId())).thenReturn(Optional.of(freshEvent));

        outboxEventProcessor.processEvent(staleEvent);

        verify(rabbitOutboxPublisher, never()).publish(any(OutboxEvent.class));
        verify(outboxEventPort, never()).save(freshEvent);
    }

    @Test
    void shouldMarkAsFailedWhenPublishThrowsAndRetryNotExhausted() {
        OutboxEvent event = claimOneEvent();
        event.setAttemptCount(0);
        event.setMaxAttempts(3);

        LocalDateTime nextRetryAt = LocalDateTime.now().plusSeconds(10);
        when(outboxEventPort.findByEventId(event.getEventId())).thenReturn(Optional.of(event));
        when(outboxRetryPolicy.isExhausted(1, 3)).thenReturn(false);
        when(outboxRetryPolicy.nextRetryAt(1)).thenReturn(nextRetryAt);
        doThrow(new RuntimeException("rabbit down")).when(rabbitOutboxPublisher).publish(event);

        outboxEventProcessor.processEvent(event);

        assertEquals(OutboxStatus.FAILED, event.getStatus());
        assertEquals(1, event.getAttemptCount());
        assertEquals(nextRetryAt, event.getNextRetryAt());
        assertNotNull(event.getLastError());
        assertTrue(event.getLastError().contains("RuntimeException: rabbit down"));
        assertEquals(null, event.getLockedBy());
    }

    @Test
    void shouldMarkAsDeadWhenRetryIsExhausted() {
        OutboxEvent event = claimOneEvent();
        event.setAttemptCount(0);
        event.setMaxAttempts(1);

        when(outboxEventPort.findByEventId(event.getEventId())).thenReturn(Optional.of(event));
        when(outboxRetryPolicy.isExhausted(1, 1)).thenReturn(true);
        doThrow(new RuntimeException("erro final")).when(rabbitOutboxPublisher).publish(event);

        outboxEventProcessor.processEvent(event);

        assertEquals(OutboxStatus.DEAD, event.getStatus());
        assertEquals(1, event.getAttemptCount());
        assertNotNull(event.getLastError());
        assertTrue(event.getLastError().contains("RuntimeException: erro final"));
        assertEquals(null, event.getLockedBy());
    }

    private OutboxEvent claimOneEvent() {
        OutboxEvent event = newOutboxEvent(OutboxStatus.NEW);
        when(outboxEventPort.findPendingForRelay(anyList(), any(LocalDateTime.class), anyInt()))
                .thenReturn(List.of(event));
        when(outboxEventPort.save(any(OutboxEvent.class))).thenAnswer(invocation -> invocation.getArgument(0));
        outboxEventProcessor.claimBatch(1);
        return event;
    }

    private OutboxEvent newOutboxEvent(OutboxStatus status) {
        return newOutboxEventWithEventId(UUID.randomUUID(), status);
    }

    private OutboxEvent newOutboxEventWithEventId(UUID eventId, OutboxStatus status) {
        return OutboxEvent.builder()
                .id(1L)
                .eventId(eventId)
                .correlationId(UUID.randomUUID())
                .producerService("humanizar-nucleo")
                .exchangeName("humanizar.nucleo.event")
                .routingKey("ev.nucleo.test.v1")
                .aggregateType("nucleo")
                .aggregateId(UUID.randomUUID())
                .eventVersion((short) 1)
                .payload("{\"ok\":true}")
                .status(status)
                .attemptCount(0)
                .maxAttempts(3)
                .nextRetryAt(LocalDateTime.now())
                .createdAt(LocalDateTime.now())
                .build();
    }
}
