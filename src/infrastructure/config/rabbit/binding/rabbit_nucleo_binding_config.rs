use crate::application::catalog::{ExchangeCatalog, QueueCatalog, RoutingKeyCatalog};
use crate::domain::exception::TechnicalError;
use crate::infrastructure::config::rabbit::rabbit_error;
use lapin::Channel;
use lapin::options::QueueBindOptions;
use lapin::types::FieldTable;

pub(crate) struct RabbitNucleoBindingConfig;

impl RabbitNucleoBindingConfig {
    pub async fn declare(channel: &Channel) -> Result<(), TechnicalError> {
        channel
            .queue_bind(
                QueueCatalog::NUCLEO_RETRIEVE.into(),
                ExchangeCatalog::NUCLEO_COMMAND.into(),
                RoutingKeyCatalog::NUCLEO_RETRIEVE_V1.into(),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(rabbit_error)
    }
}
