package com.humanizar.nucleo.infrastructure.config;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemRegisterDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemRetrieveDTO;
import com.humanizar.nucleo.application.dto.abordagem.AbordagemUpdateDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoRegisterDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoRetrieveDTO;
import com.humanizar.nucleo.application.dto.nucleo.NucleoUpdateDTO;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.infrastructure.controller.dto.ErrorResponseDTO;
import com.humanizar.nucleo.infrastructure.controller.dto.SuccessResponseDTO;
import org.springframework.aot.hint.MemberCategory;
import org.springframework.aot.hint.RuntimeHints;
import org.springframework.aot.hint.RuntimeHintsRegistrar;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.annotation.ImportRuntimeHints;

@Configuration
@ImportRuntimeHints(ObjectMapperConfig.NucleoRuntimeHints.class)
public class ObjectMapperConfig {

    @Bean
    public ObjectMapper objectMapper() {
        ObjectMapper mapper = new ObjectMapper();
        mapper.registerModule(new JavaTimeModule());
        mapper.disable(SerializationFeature.WRITE_DATES_AS_TIMESTAMPS);
        mapper.disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES);
        return mapper;
    }

    public static class NucleoRuntimeHints implements RuntimeHintsRegistrar {

        @Override
        public void registerHints(RuntimeHints hints, ClassLoader classLoader) {
            registerJsonBinding(hints, NucleoRegisterDTO.class);
            registerJsonBinding(hints, NucleoUpdateDTO.class);
            registerJsonBinding(hints, NucleoRetrieveDTO.class);
            registerJsonBinding(hints, AbordagemRegisterDTO.class);
            registerJsonBinding(hints, AbordagemUpdateDTO.class);
            registerJsonBinding(hints, AbordagemRetrieveDTO.class);
            registerJsonBinding(hints, ErrorResponseDTO.class);
            registerJsonBinding(hints, SuccessResponseDTO.class);
            registerJsonBinding(hints, ReasonCode.class);
        }

        private void registerJsonBinding(RuntimeHints hints, Class<?> type) {
            hints.reflection().registerType(type, MemberCategory.values());
        }
    }
}
