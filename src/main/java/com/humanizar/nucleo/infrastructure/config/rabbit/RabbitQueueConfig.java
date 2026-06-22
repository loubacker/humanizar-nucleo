package com.humanizar.nucleo.infrastructure.config.rabbit;

import org.springframework.amqp.core.Queue;
import org.springframework.amqp.core.QueueBuilder;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import com.humanizar.nucleo.application.catalog.QueueCatalog;

@Configuration
public class RabbitQueueConfig {

    private static final int INBOUND_DELIVERY_LIMIT = 3;

    /**
     * Declara a dead-letter queue da fila de comandos do nucleo
     * ({@code humanizar.nucleo.retrieve.dlq}, quorum).
     */
    @Bean
    public Queue nucleoRetrieveDlq() {
        return QueueBuilder.durable(QueueCatalog.NUCLEO_RETRIEVE_DLQ)
                .quorum()
                .build();
    }

    /**
     * Declara a fila inbound de comandos do nucleo ({@code humanizar.nucleo.retrieve},
     * quorum), com {@code x-delivery-limit=3} e dead-letter para {@link #nucleoRetrieveDlq()}.
     */
    @Bean
    public Queue nucleoRetrieveQueue() {
        return QueueBuilder.durable(QueueCatalog.NUCLEO_RETRIEVE)
                .quorum()
                .withArgument("x-delivery-limit", INBOUND_DELIVERY_LIMIT)
                .deadLetterExchange("")
                .deadLetterRoutingKey(QueueCatalog.NUCLEO_RETRIEVE_DLQ)
                .build();
    }
}
