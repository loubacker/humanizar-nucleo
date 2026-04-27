package com.humanizar.nucleo.infrastructure.controller.handler;

import com.humanizar.nucleo.domain.exception.NucleoException;
import com.humanizar.nucleo.domain.model.enums.ReasonCode;
import com.humanizar.nucleo.infrastructure.controller.dto.ErrorResponseDTO;
import jakarta.servlet.http.HttpServletRequest;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.security.core.AuthenticationException;
import org.springframework.web.bind.MissingServletRequestParameterException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;
import org.springframework.web.method.annotation.MethodArgumentTypeMismatchException;

@RestControllerAdvice
public class NucleoExceptionHandler {

    private static final Logger log = LoggerFactory.getLogger(NucleoExceptionHandler.class);

    @ExceptionHandler(NucleoException.class)
    public ResponseEntity<ErrorResponseDTO> handleNucleoException(
            NucleoException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = exception.reasonCode();
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(AuthenticationException.class)
    public ResponseEntity<ErrorResponseDTO> handleAuthenticationException(
            AuthenticationException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.AUTHENTICATION_REQUIRED;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(AccessDeniedException.class)
    public ResponseEntity<ErrorResponseDTO> handleAccessDeniedException(
            AccessDeniedException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.AUTHORIZATION_DENIED;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(MissingServletRequestParameterException.class)
    public ResponseEntity<ErrorResponseDTO> handleMissingServletRequestParameterException(
            MissingServletRequestParameterException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.REQUEST_PARAMETER_MISSING;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(MethodArgumentTypeMismatchException.class)
    public ResponseEntity<ErrorResponseDTO> handleMethodArgumentTypeMismatchException(
            MethodArgumentTypeMismatchException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.REQUEST_PARAMETER_INVALID;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(HttpMessageNotReadableException.class)
    public ResponseEntity<ErrorResponseDTO> handleHttpMessageNotReadableException(
            HttpMessageNotReadableException exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.REQUEST_BODY_INVALID;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<ErrorResponseDTO> handleException(
            Exception exception,
            HttpServletRequest request) {
        ReasonCode reasonCode = ReasonCode.UNEXPECTED_ERROR;
        logHandledException(reasonCode, request, exception);
        return buildResponse(reasonCode, request);
    }

    private ResponseEntity<ErrorResponseDTO> buildResponse(ReasonCode reasonCode, HttpServletRequest request) {
        HttpStatus status = HttpStatus.valueOf(reasonCode.httpStatus());
        ErrorResponseDTO response = new ErrorResponseDTO(
                reasonCode.httpStatus(),
                status.getReasonPhrase(),
                reasonCode.name(),
                reasonCode.message(),
                request.getRequestURI());
        return ResponseEntity.status(status).body(response);
    }

    private void logHandledException(ReasonCode reasonCode, HttpServletRequest request, Exception exception) {
        if (reasonCode.httpStatus() >= 500) {
            log.error("Falha tratada no nucleo-service. reasonCode={}, path={}",
                    reasonCode.name(),
                    request.getRequestURI(),
                    exception);
            return;
        }

        log.warn("Falha tratada no nucleo-service. reasonCode={}, path={}, message={}",
                reasonCode.name(),
                request.getRequestURI(),
                exception.getMessage());
    }
}
