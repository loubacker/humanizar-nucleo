use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use tokio_postgres::error::SqlState;

pub(crate) fn database_error(error: tokio_postgres::Error) -> NucleoException {
    let reason = match error.as_db_error() {
        Some(details)
            if (details.code() == &SqlState::FOREIGN_KEY_VIOLATION
                || details.code() == &SqlState::RESTRICT_VIOLATION)
                && details.constraint() == Some("fk_abordagem_nucleo") =>
        {
            ReasonCode::NucleoHasAbordagens
        }
        _ => ReasonCode::DatabaseAccessFailed,
    };
    let transient = error.is_closed()
        || error.code().is_some_and(|code| {
            matches!(code.code(), "40001" | "40P01" | "57P01" | "57P02" | "57P03")
                || code.code().starts_with("08")
        });
    NucleoException::with_retryable_source(reason, transient, error)
}

pub(crate) fn abordagem_create_error(error: tokio_postgres::Error) -> NucleoException {
    let reason = match error.as_db_error() {
        Some(details)
            if details.code() == &SqlState::FOREIGN_KEY_VIOLATION
                && details.constraint() == Some("fk_abordagem_nucleo") =>
        {
            ReasonCode::NucleoNotFound
        }
        _ => ReasonCode::DatabaseAccessFailed,
    };
    let transient = error.is_closed()
        || error.code().is_some_and(|code| {
            matches!(code.code(), "40001" | "40P01" | "57P01" | "57P02" | "57P03")
                || code.code().starts_with("08")
        });
    NucleoException::with_retryable_source(reason, transient, error)
}
