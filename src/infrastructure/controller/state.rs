use crate::application::service::{AbordagemService, NucleoService};
use crate::domain::port::{AbordagemPort, NucleoPort};
use crate::infrastructure::adapter::{AbordagemAdapter, NucleoAdapter};
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::persistence::repository::{AbordagemRepository, NucleoRepository};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

#[derive(Clone)]
pub struct ApplicationState {
    pub(crate) nucleos: Arc<NucleoService>,
    pub(crate) abordagens: Arc<AbordagemService>,
    pub(crate) messaging_ready: Arc<AtomicBool>,
    pub(crate) database: Option<DatabaseConfig>,
}

impl ApplicationState {
    pub fn new(nucleos: Arc<NucleoService>, abordagens: Arc<AbordagemService>) -> Self {
        Self {
            nucleos,
            abordagens,
            messaging_ready: Arc::new(AtomicBool::new(true)),
            database: None,
        }
    }

    pub fn from_database(database: DatabaseConfig, readiness: Arc<AtomicBool>) -> Self {
        let health_database = database.clone();
        let nucleos: Arc<dyn NucleoPort> =
            Arc::new(NucleoAdapter::new(NucleoRepository::new(database.clone())));
        let abordagens: Arc<dyn AbordagemPort> =
            Arc::new(AbordagemAdapter::new(AbordagemRepository::new(database)));
        let mut state = Self::new(
            Arc::new(NucleoService::new(nucleos.clone(), abordagens.clone())),
            Arc::new(AbordagemService::new(abordagens, nucleos)),
        );
        state.messaging_ready = readiness;
        state.database = Some(health_database);
        state
    }
}
