package com.humanizar.nucleo.infrastructure.config.rabbit;

import org.springframework.amqp.core.TopicExchange;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

import com.humanizar.nucleo.application.catalog.ExchangeCatalog;

@Configuration
public class RabbitExchangeConfig {

    /**
     * Declara a exchange {@code humanizar.nucleo.command} (topic, durável): canal de
     * entrada dos comandos do nucleo (ex.: retrieve do nome do núcleo).
     */
    @Bean
    public TopicExchange nucleoCommandExchange() {
        return new TopicExchange(ExchangeCatalog.NUCLEO_COMMAND, true, false);
    }

    /**
     * Declara a exchange {@code humanizar.nucleo.event} (topic, durável): canal de saída
     * dos callbacks do nucleo (retrieved/rejected), consumidos pelos solicitantes.
     */
    @Bean
    public TopicExchange nucleoEventExchange() {
        return new TopicExchange(ExchangeCatalog.NUCLEO_EVENT, true, false);
    }
}
