package com.humanizar.nucleo.application.messaging.inbound.handler;

import java.time.LocalDateTime;
import java.util.UUID;

import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import com.humanizar.nucleo.application.catalog.RoutingKeyCatalog;
import com.humanizar.nucleo.application.messaging.inbound.dto.InboundEnvelopeDTO;
import com.humanizar.nucleo.application.messaging.inbound.dto.NucleoRetrieveCommandDTO;
import com.humanizar.nucleo.application.messaging.inbound.mapper.NucleoCommandMapper;
import com.humanizar.nucleo.application.messaging.outbound.dto.NucleoRejectedDTO;
import com.humanizar.nucleo.application.messaging.outbound.dto.NucleoRetrievedDTO;
import com.humanizar.nucleo.application.messaging.outbound.dto.OutboundEnvelopeDTO;
import com.humanizar.nucleo.application.messaging.outbound.publisher.OutboxEventPublisher;
import com.humanizar.nucleo.domain.model.Nucleo;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.NucleoPort;

@Service
public class NucleoRetrieveHandler {

    private static final String PRODUCER_SERVICE = "humanizar-nucleo";
    private static final String AGGREGATE_TYPE = "Nucleo";
    private static final int EVENT_VERSION = 1;

    private final NucleoCommandMapper nucleoCommandMapper;
    private final NucleoPort nucleoPort;
    private final OutboxEventPublisher outboxEventPublisher;

    public NucleoRetrieveHandler(
            NucleoCommandMapper nucleoCommandMapper,
            NucleoPort nucleoPort,
            OutboxEventPublisher outboxEventPublisher) {
        this.nucleoCommandMapper = nucleoCommandMapper;
        this.nucleoPort = nucleoPort;
        this.outboxEventPublisher = outboxEventPublisher;
    }

    @Transactional
    public EventOutcome handle(InboundEnvelopeDTO<Object> envelope) {
        NucleoRetrieveCommandDTO command = nucleoCommandMapper.toCommand(envelope);
        Nucleo nucleo = nucleoPort.findById(command.nucleoId()).orElse(null);

        if (nucleo == null) {
            publishRejected(envelope, command, ReasonCode.NUCLEO_NOT_FOUND, ReasonCode.NUCLEO_NOT_FOUND.message());
            return EventOutcome.success();
        }

        publishRetrieved(envelope, command, nucleo.getNome());
        return EventOutcome.success();
    }

    private void publishRetrieved(
            InboundEnvelopeDTO<Object> envelope,
            NucleoRetrieveCommandDTO command,
            String nome) {
        UUID callbackEventId = UUID.randomUUID();
        NucleoRetrievedDTO payload = new NucleoRetrievedDTO(
                envelope.eventId(),
                command.sourceEventId(),
                command.nucleoId(),
                nome);
        OutboundEnvelopeDTO<NucleoRetrievedDTO> outboundEnvelope = new OutboundEnvelopeDTO<>(
                callbackEventId,
                envelope.correlationId(),
                PRODUCER_SERVICE,
                command.callbackExchange(),
                RoutingKeyCatalog.NUCLEO_RETRIEVED_V1,
                AGGREGATE_TYPE,
                command.nucleoId(),
                EVENT_VERSION,
                LocalDateTime.now(),
                envelope.actorId(),
                envelope.userAgent(),
                envelope.originIp(),
                payload);

        outboxEventPublisher.publish(
                command.callbackExchange(),
                RoutingKeyCatalog.NUCLEO_RETRIEVED_V1,
                AGGREGATE_TYPE,
                command.nucleoId(),
                callbackEventId,
                envelope.correlationId(),
                outboundEnvelope,
                envelope.actorId(),
                envelope.userAgent(),
                envelope.originIp());
    }

    private void publishRejected(
            InboundEnvelopeDTO<Object> envelope,
            NucleoRetrieveCommandDTO command,
            ReasonCode reasonCode,
            String message) {
        UUID callbackEventId = UUID.randomUUID();
        NucleoRejectedDTO payload = new NucleoRejectedDTO(
                envelope.eventId(),
                command.sourceEventId(),
                command.nucleoId(),
                reasonCode.name(),
                message);
        OutboundEnvelopeDTO<NucleoRejectedDTO> outboundEnvelope = new OutboundEnvelopeDTO<>(
                callbackEventId,
                envelope.correlationId(),
                PRODUCER_SERVICE,
                command.callbackExchange(),
                RoutingKeyCatalog.NUCLEO_REJECTED_V1,
                AGGREGATE_TYPE,
                command.nucleoId(),
                EVENT_VERSION,
                LocalDateTime.now(),
                envelope.actorId(),
                envelope.userAgent(),
                envelope.originIp(),
                payload);

        outboxEventPublisher.publish(
                command.callbackExchange(),
                RoutingKeyCatalog.NUCLEO_REJECTED_V1,
                AGGREGATE_TYPE,
                command.nucleoId(),
                callbackEventId,
                envelope.correlationId(),
                outboundEnvelope,
                envelope.actorId(),
                envelope.userAgent(),
                envelope.originIp());
    }
}
