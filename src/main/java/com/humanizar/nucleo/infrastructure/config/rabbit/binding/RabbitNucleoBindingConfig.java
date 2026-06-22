package com.humanizar.nucleo.infrastructure.config.rabbit.binding;

import org.springframework.amqp.core.Binding;
import org.springframework.amqp.core.BindingBuilder;
import org.springframework.amqp.core.Queue;
import org.springframework.amqp.core.TopicExchange;
import org.springframework.beans.factory.annotation.Qualifier;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import com.humanizar.nucleo.application.catalog.RoutingKeyCatalog;

@Configuration
public class RabbitNucleoBindingConfig {

    /**
     * Binda a fila de comandos {@code humanizar.nucleo.retrieve} à exchange
     * {@code humanizar.nucleo.command} pela routing key {@code cmd.nucleo.retrieve.v1},
     * entregando ao nucleo os comandos de busca do nome do núcleo.
     */
    @Bean
    public Binding nucleoRetrieveBinding(
            @Qualifier("nucleoRetrieveQueue") Queue queue,
            @Qualifier("nucleoCommandExchange") TopicExchange exchange) {
        return BindingBuilder.bind(queue)
                .to(exchange)
                .with(RoutingKeyCatalog.NUCLEO_RETRIEVE_V1);
    }
}
