use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::domain::model::enums::ReasonCode;

use super::PersistenceException;
use super::{ReasonCodeException, TechnicalError};

#[derive(Debug)]
pub struct NucleoException {
    reason_code: ReasonCode,
    details: TechnicalError,
    retryable: bool,
}

impl NucleoException {
    pub fn new(reason_code: ReasonCode) -> Self {
        Self {
            reason_code,
            details: TechnicalError::new(reason_code.message()),
            retryable: reason_code.is_retryable(),
        }
    }

    pub fn with_message(reason_code: ReasonCode, message: impl Into<String>) -> Self {
        Self {
            reason_code,
            details: TechnicalError::new(resolve_message(reason_code, message.into())),
            retryable: reason_code.is_retryable(),
        }
    }

    pub fn with_source<E>(reason_code: ReasonCode, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            reason_code,
            details: TechnicalError::with_source(reason_code.message(), source),
            retryable: reason_code.is_retryable(),
        }
    }

    pub fn with_message_and_source<E>(
        reason_code: ReasonCode,
        message: impl Into<String>,
        source: E,
    ) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            reason_code,
            details: TechnicalError::with_source(
                resolve_message(reason_code, message.into()),
                source,
            ),
            retryable: reason_code.is_retryable(),
        }
    }

    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    pub fn message(&self) -> &str {
        self.details.message()
    }

    pub const fn is_retryable(&self) -> bool {
        self.retryable
    }

    pub fn with_retryable_source(
        reason_code: ReasonCode,
        retryable: bool,
        source: impl Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            reason_code,
            details: TechnicalError::with_source(reason_code.message(), source),
            retryable,
        }
    }
}

impl Display for NucleoException {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.details, formatter)
    }
}

impl Error for NucleoException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.details.source()
    }
}

impl ReasonCodeException for NucleoException {
    fn reason_code(&self) -> ReasonCode {
        NucleoException::reason_code(self)
    }

    fn message(&self) -> &str {
        NucleoException::message(self)
    }
}

impl From<PersistenceException> for NucleoException {
    fn from(exception: PersistenceException) -> Self {
        Self::with_retryable_source(
            ReasonCode::PersistenceFailure,
            exception.is_retryable(),
            exception,
        )
    }
}

fn resolve_message(reason_code: ReasonCode, message: String) -> String {
    if message.trim().is_empty() {
        return reason_code.message().to_owned();
    }

    message
}
