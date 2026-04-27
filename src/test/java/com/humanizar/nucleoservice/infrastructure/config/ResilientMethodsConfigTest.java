package com.humanizar.nucleoservice.infrastructure.config;

import com.humanizar.nucleo.infrastructure.config.ResilientMethodsConfig;
import jakarta.persistence.QueryTimeoutException;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.springframework.dao.RecoverableDataAccessException;
import org.springframework.dao.TransientDataAccessResourceException;
import org.springframework.transaction.CannotCreateTransactionException;

import java.lang.reflect.Method;

class ResilientMethodsConfigTest {

    private final ResilientMethodsConfig.RetrieveTransientRetryPredicate predicate =
            new ResilientMethodsConfig.RetrieveTransientRetryPredicate();

    @Test
    void shouldRetryForTransientDataAccessException() throws NoSuchMethodException {
        Method method = getClass().getDeclaredMethod("sampleMethod");

        boolean shouldRetry = predicate.shouldRetry(
                method,
                new TransientDataAccessResourceException("falha transiente"));

        Assertions.assertTrue(shouldRetry);
    }

    @Test
    void shouldRetryForNestedRecoverableException() throws NoSuchMethodException {
        Method method = getClass().getDeclaredMethod("sampleMethod");

        boolean shouldRetry = predicate.shouldRetry(
                method,
                new IllegalStateException(
                        "erro externo",
                        new RecoverableDataAccessException("falha recuperavel")));

        Assertions.assertTrue(shouldRetry);
    }

    @Test
    void shouldRetryForCannotCreateTransactionException() throws NoSuchMethodException {
        Method method = getClass().getDeclaredMethod("sampleMethod");

        boolean shouldRetry = predicate.shouldRetry(
                method,
                new CannotCreateTransactionException("transacao indisponivel"));

        Assertions.assertTrue(shouldRetry);
    }

    @Test
    void shouldRetryForQueryTimeoutException() throws NoSuchMethodException {
        Method method = getClass().getDeclaredMethod("sampleMethod");

        boolean shouldRetry = predicate.shouldRetry(
                method,
                new QueryTimeoutException("timeout de consulta"));

        Assertions.assertTrue(shouldRetry);
    }

    @Test
    void shouldNotRetryForNonTransientException() throws NoSuchMethodException {
        Method method = getClass().getDeclaredMethod("sampleMethod");

        boolean shouldRetry = predicate.shouldRetry(method, new IllegalArgumentException("erro definitivo"));

        Assertions.assertFalse(shouldRetry);
    }

    private void sampleMethod() {
    }
}
