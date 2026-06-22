package com.humanizar.nucleo.application.messaging.outbound.publisher;

import java.util.UUID;

import org.springframework.stereotype.Component;

import com.humanizar.nucleo.application.messaging.outbound.mapper.OutboxEventMapper;
import com.humanizar.nucleo.domain.model.OutboxEvent;
import com.humanizar.nucleo.domain.port.OutboxEventPort;

@Component
public class OutboxEventPublisher {

    private final OutboxEventPort outboxEventPort;
    private final OutboxEventMapper outboxEventMapper;

    public OutboxEventPublisher(OutboxEventPort outboxEventPort, OutboxEventMapper outboxEventMapper) {
        this.outboxEventPort = outboxEventPort;
        this.outboxEventMapper = outboxEventMapper;
    }

    public void publish(String exchangeName,
            String routingKey,
            String aggregateType,
            UUID aggregateId,
            UUID eventId,
            UUID correlationId,
            Object payload,
            UUID actorId,
            String userAgent,
            String originIp) {
        OutboxEvent event = outboxEventMapper.toOutboxEvent(
                exchangeName,
                routingKey,
                aggregateType,
                aggregateId,
                eventId,
                correlationId,
                payload,
                actorId,
                userAgent,
                originIp);
        outboxEventPort.save(event);
    }
}
