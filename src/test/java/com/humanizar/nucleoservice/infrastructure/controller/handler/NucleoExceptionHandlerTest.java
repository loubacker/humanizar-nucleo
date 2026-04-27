package com.humanizar.nucleoservice.infrastructure.controller.handler;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.infrastructure.controller.dto.ErrorResponseDTO;
import com.humanizar.nucleo.infrastructure.controller.handler.NucleoExceptionHandler;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.springframework.http.ResponseEntity;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.mock.http.MockHttpInputMessage;
import org.springframework.mock.web.MockHttpServletRequest;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.authentication.BadCredentialsException;
import org.springframework.web.bind.MissingServletRequestParameterException;
import org.springframework.web.method.annotation.MethodArgumentTypeMismatchException;

import java.util.UUID;

class NucleoExceptionHandlerTest {

    private final NucleoExceptionHandler handler = new NucleoExceptionHandler();

    @Test
    void shouldReturnReasonCodeForNucleoException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleo/123");

        ResponseEntity<ErrorResponseDTO> response = handler.handleNucleoException(
                new NucleoException(ReasonCode.NUCLEO_NOT_FOUND),
                request);

        Assertions.assertEquals(404, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("NUCLEO_NOT_FOUND", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnReasonCodeForAuthenticationException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleos");

        ResponseEntity<ErrorResponseDTO> response = handler.handleAuthenticationException(
                new BadCredentialsException("token invalido"),
                request);

        Assertions.assertEquals(401, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("AUTHENTICATION_REQUIRED", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnReasonCodeForAccessDeniedException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleo/register");

        ResponseEntity<ErrorResponseDTO> response = handler.handleAccessDeniedException(
                new AccessDeniedException("acesso negado"),
                request);

        Assertions.assertEquals(403, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("AUTHORIZATION_DENIED", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnReasonCodeForMissingRequestParameterException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleos");

        ResponseEntity<ErrorResponseDTO> response = handler.handleMissingServletRequestParameterException(
                new MissingServletRequestParameterException("id", "UUID"),
                request);

        Assertions.assertEquals(400, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("REQUEST_PARAMETER_MISSING", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnReasonCodeForMethodArgumentTypeMismatchException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleo/abc");

        ResponseEntity<ErrorResponseDTO> response = handler.handleMethodArgumentTypeMismatchException(
                new MethodArgumentTypeMismatchException("abc", UUID.class, "id", null, null),
                request);

        Assertions.assertEquals(400, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("REQUEST_PARAMETER_INVALID", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnReasonCodeForHttpMessageNotReadableException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleo/register");

        ResponseEntity<ErrorResponseDTO> response = handler.handleHttpMessageNotReadableException(
                new HttpMessageNotReadableException("json invalido", new MockHttpInputMessage(new byte[0])),
                request);

        Assertions.assertEquals(400, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("REQUEST_BODY_INVALID", response.getBody().reasonCode());
    }

    @Test
    void shouldReturnUnexpectedErrorForUnhandledException() {
        MockHttpServletRequest request = createRequest("/api/v1/nucleos");

        ResponseEntity<ErrorResponseDTO> response = handler.handleException(
                new IllegalStateException("falha interna"),
                request);

        Assertions.assertEquals(500, response.getStatusCode().value());
        Assertions.assertNotNull(response.getBody());
        Assertions.assertEquals("UNEXPECTED_ERROR", response.getBody().reasonCode());
    }

    private MockHttpServletRequest createRequest(String requestUri) {
        MockHttpServletRequest request = new MockHttpServletRequest();
        request.setRequestURI(requestUri);
        return request;
    }
}
