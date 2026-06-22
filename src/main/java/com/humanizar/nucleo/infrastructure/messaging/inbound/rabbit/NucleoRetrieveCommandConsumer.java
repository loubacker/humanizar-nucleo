package com.humanizar.nucleo.infrastructure.messaging.inbound.rabbit;

import java.io.IOException;
import java.time.LocalDateTime;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.core.Message;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;

import com.humanizar.nucleo.application.catalog.ConsumerCatalog;
import com.humanizar.nucleo.application.catalog.QueueCatalog;
import com.humanizar.nucleo.application.catalog.RoutingKeyCatalog;
import com.humanizar.nucleo.application.messaging.inbound.dto.InboundEnvelopeDTO;
import com.humanizar.nucleo.application.messaging.inbound.handler.EventOutcome;
import com.humanizar.nucleo.application.messaging.inbound.handler.NucleoRetrieveHandler;
import com.humanizar.nucleo.application.messaging.inbound.mapper.EnvelopeInboundMapper;
import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.ProcessedEvent;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.domain.port.ProcessedEventPort;
import com.humanizar.nucleo.infrastructure.config.rabbit.RabbitAcknowledgementConfig;
import com.humanizar.nucleo.infrastructure.messaging.inbound.idempotency.ProcessedEventGuard;
import com.rabbitmq.client.Channel;

@Component
public class NucleoRetrieveCommandConsumer {

    private static final Logger log = LoggerFactory.getLogger(NucleoRetrieveCommandConsumer.class);

    private final EnvelopeInboundMapper envelopeInboundMapper;
    private final ProcessedEventGuard processedEventGuard;
    private final ProcessedEventPort processedEventPort;
    private final RabbitAcknowledgementConfig rabbitAcknowledgementConfig;
    private final NucleoRetrieveHandler nucleoRetrieveHandler;

    public NucleoRetrieveCommandConsumer(
            EnvelopeInboundMapper envelopeInboundMapper,
            ProcessedEventGuard processedEventGuard,
            ProcessedEventPort processedEventPort,
            RabbitAcknowledgementConfig rabbitAcknowledgementConfig,
            NucleoRetrieveHandler nucleoRetrieveHandler) {
        this.envelopeInboundMapper = envelopeInboundMapper;
        this.processedEventGuard = processedEventGuard;
        this.processedEventPort = processedEventPort;
        this.rabbitAcknowledgementConfig = rabbitAcknowledgementConfig;
        this.nucleoRetrieveHandler = nucleoRetrieveHandler;
    }

    @RabbitListener(queues = QueueCatalog.NUCLEO_RETRIEVE, containerFactory = "rabbitListenerContainerFactory")
    public void onMessage(Message message, Channel channel) throws IOException {
        long deliveryTag = message.getMessageProperties().getDeliveryTag();
        String routingKey = message.getMessageProperties().getReceivedRoutingKey();
        InboundEnvelopeDTO<Object> envelope = null;

        try {
            envelope = envelopeInboundMapper.parseEnvelope(message.getBody());
            envelopeInboundMapper.validate(envelope);
            processedEventGuard.ensureNotProcessed(
                    ConsumerCatalog.RETRIEVE_CONSUMER,
                    envelope.eventId(),
                    envelopeInboundMapper.correlationIdAsString(envelope));

            EventOutcome outcome = dispatch(envelope, routingKey);
            saveProcessedEvent(envelope, routingKey, outcome);
            rabbitAcknowledgementConfig.ack(channel, deliveryTag, context(routingKey, envelope));
        } catch (NucleoException exception) {
            handleNucleoException(channel, deliveryTag, routingKey, envelope, exception);
        } catch (RuntimeException runtimeException) {
            log.error("Falha inesperada ao consumir comando de retrieve do nucleo. routingKey={}", routingKey,
                    runtimeException);
            rabbitAcknowledgementConfig.nackRetry(channel, deliveryTag, "routingKey=" + routingKey);
        }
    }

    private EventOutcome dispatch(InboundEnvelopeDTO<Object> envelope, String routingKey) {
        if (!RoutingKeyCatalog.NUCLEO_RETRIEVE_V1.equals(routingKey)) {
            throw new NucleoException(
                    ReasonCode.UNSUPPORTED_ROUTING_KEY,
                    "Routing key nao suportada: " + routingKey,
                    envelopeInboundMapper.correlationIdAsString(envelope));
        }

        return nucleoRetrieveHandler.handle(envelope);
    }

    private void handleNucleoException(
            Channel channel,
            long deliveryTag,
            String routingKey,
            InboundEnvelopeDTO<Object> envelope,
            NucleoException exception) throws IOException {
        if (ReasonCode.DUPLICATE_EVENT.equals(exception.getReasonCode())) {
            log.info("Evento duplicado ignorado. eventId={}, routingKey={}",
                    envelope != null ? envelope.eventId() : null, routingKey);
            rabbitAcknowledgementConfig.ack(channel, deliveryTag, context(routingKey, envelope));
            return;
        }

        if (exception.isRetryable()) {
            rabbitAcknowledgementConfig.nackRetry(channel, deliveryTag, context(routingKey, envelope));
            return;
        }

        saveFailedProcessedEvent(envelope, routingKey, exception);
        rabbitAcknowledgementConfig.nackDeadLetter(channel, deliveryTag, context(routingKey, envelope));
    }

    private void saveProcessedEvent(InboundEnvelopeDTO<Object> envelope, String routingKey, EventOutcome outcome) {
        processedEventPort.save(ProcessedEvent.builder()
                .consumerName(ConsumerCatalog.RETRIEVE_CONSUMER)
                .eventId(envelope.eventId())
                .correlationId(envelope.correlationId())
                .eventType(routingKey)
                .aggregateType(envelope.aggregateType())
                .aggregateId(envelope.aggregateId())
                .actorId(envelope.actorId())
                .userAgent(envelope.userAgent())
                .originIp(envelope.originIp())
                .processedAt(LocalDateTime.now())
                .result(outcome.result())
                .reasonCode(outcome.reasonCode())
                .errorMessage(outcome.errorMessage())
                .build());
    }

    private void saveFailedProcessedEvent(
            InboundEnvelopeDTO<Object> envelope,
            String routingKey,
            NucleoException exception) {
        if (envelope == null || envelope.eventId() == null || envelope.correlationId() == null) {
            return;
        }

        saveProcessedEvent(envelope, routingKey,
                EventOutcome.failed(exception.getReasonCode(), exception.getMessage()));
    }

    private String context(String routingKey, InboundEnvelopeDTO<Object> envelope) {
        if (envelope == null) {
            return "routingKey=" + routingKey;
        }
        return "eventId=" + envelope.eventId() + ",routingKey=" + routingKey;
    }
}
