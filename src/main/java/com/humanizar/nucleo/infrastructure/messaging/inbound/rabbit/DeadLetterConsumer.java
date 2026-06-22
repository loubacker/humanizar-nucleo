package com.humanizar.nucleo.infrastructure.messaging.inbound.rabbit;

import java.io.IOException;
import java.util.List;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.amqp.core.Message;
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;

import com.humanizar.nucleo.application.catalog.QueueCatalog;
import com.humanizar.nucleo.infrastructure.config.rabbit.RabbitAcknowledgementConfig;
import com.rabbitmq.client.Channel;

@Component
public class DeadLetterConsumer {

    private static final Logger log = LoggerFactory.getLogger(DeadLetterConsumer.class);

    private final RabbitAcknowledgementConfig rabbitAcknowledgementConfig;

    public DeadLetterConsumer(RabbitAcknowledgementConfig rabbitAcknowledgementConfig) {
        this.rabbitAcknowledgementConfig = rabbitAcknowledgementConfig;
    }

    @RabbitListener(queues = {
            QueueCatalog.NUCLEO_RETRIEVE_DLQ
    }, containerFactory = "rabbitListenerContainerFactory")
    public void onDeadLetter(Message message, Channel channel) throws IOException {
        long deliveryTag = message.getMessageProperties().getDeliveryTag();
        String originalRoutingKey = extractOriginalRoutingKey(message);
        String queue = message.getMessageProperties().getConsumerQueue();

        log.error("Mensagem enviada para DLQ. queue={}, originalRoutingKey={}, messageId={}",
                queue,
                originalRoutingKey,
                message.getMessageProperties().getMessageId());

        rabbitAcknowledgementConfig.ack(channel, deliveryTag, context(queue, originalRoutingKey));
    }

    private String extractOriginalRoutingKey(Message message) {
        List<Map<String, ?>> xDeath = message.getMessageProperties().getXDeathHeader();
        if (xDeath != null && !xDeath.isEmpty()) {
            Map<String, ?> first = xDeath.getFirst();
            Object routingKeys = first.get("routing-keys");
            if (routingKeys instanceof List<?> keys && !keys.isEmpty()) {
                return keys.getFirst().toString();
            }
        }
        return message.getMessageProperties().getReceivedRoutingKey();
    }

    private String context(String queue, String routingKey) {
        return "queue=" + queue + ",routingKey=" + routingKey;
    }
}
