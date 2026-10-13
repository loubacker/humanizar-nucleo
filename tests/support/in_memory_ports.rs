use async_trait::async_trait;
use humanizar_nucleo::application::service::{AbordagemService, NucleoService};
use humanizar_nucleo::domain::exception::NucleoException;
use humanizar_nucleo::domain::model::enums::ReasonCode;
use humanizar_nucleo::domain::model::{Abordagem, Nucleo};
use humanizar_nucleo::domain::port::{AbordagemPort, NucleoPort};
use humanizar_nucleo::infrastructure::controller::ApplicationState;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Default)]
pub struct InMemoryPorts {
    nucleos: Mutex<Vec<Nucleo>>,
    abordagens: Mutex<Vec<Abordagem>>,
    pub batch_reads: AtomicUsize,
    pub effects: AtomicUsize,
}

impl InMemoryPorts {
    pub fn state(self: &Arc<Self>) -> ApplicationState {
        ApplicationState::new(
            Arc::new(NucleoService::new(self.clone(), self.clone())),
            Arc::new(AbordagemService::new(self.clone(), self.clone())),
        )
    }
}

#[async_trait]
impl NucleoPort for InMemoryPorts {
    async fn create(&self, nucleo: Nucleo) -> Result<Nucleo, NucleoException> {
        self.effects.fetch_add(1, Ordering::SeqCst);
        self.nucleos
            .lock()
            .expect("fixture lock")
            .push(nucleo.clone());
        Ok(nucleo)
    }

    async fn update(&self, nucleo: Nucleo) -> Result<Nucleo, NucleoException> {
        let mut nucleos = self.nucleos.lock().expect("fixture lock");
        let row = nucleos
            .iter_mut()
            .find(|row| row.id() == nucleo.id())
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoNotFound))?;
        *row = nucleo.clone();
        self.effects.fetch_add(1, Ordering::SeqCst);
        Ok(nucleo)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Nucleo>, NucleoException> {
        Ok(self
            .nucleos
            .lock()
            .expect("fixture lock")
            .iter()
            .find(|row| row.id() == id)
            .cloned())
    }

    async fn find_all(&self) -> Result<Vec<Nucleo>, NucleoException> {
        Ok(self.nucleos.lock().expect("fixture lock").clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        if self
            .abordagens
            .lock()
            .expect("fixture lock")
            .iter()
            .any(|row| row.nucleo_id() == id)
        {
            return Err(NucleoException::new(ReasonCode::NucleoHasAbordagens));
        }
        let mut nucleos = self.nucleos.lock().expect("fixture lock");
        let index = nucleos
            .iter()
            .position(|row| row.id() == id)
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoNotFound))?;
        nucleos.remove(index);
        self.effects.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

#[async_trait]
impl AbordagemPort for InMemoryPorts {
    async fn create(&self, abordagem: Abordagem) -> Result<Abordagem, NucleoException> {
        self.effects.fetch_add(1, Ordering::SeqCst);
        self.abordagens
            .lock()
            .expect("fixture lock")
            .push(abordagem.clone());
        Ok(abordagem)
    }

    async fn update(&self, abordagem: Abordagem) -> Result<Abordagem, NucleoException> {
        let mut abordagens = self.abordagens.lock().expect("fixture lock");
        let row = abordagens
            .iter_mut()
            .find(|row| row.id() == abordagem.id())
            .ok_or_else(|| NucleoException::new(ReasonCode::AbordagemNotFound))?;
        *row = abordagem.clone();
        self.effects.fetch_add(1, Ordering::SeqCst);
        Ok(abordagem)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Abordagem>, NucleoException> {
        Ok(self
            .abordagens
            .lock()
            .expect("fixture lock")
            .iter()
            .find(|row| row.id() == id)
            .cloned())
    }

    async fn find_by_nucleo_ids(&self, ids: &[Uuid]) -> Result<Vec<Abordagem>, NucleoException> {
        self.batch_reads.fetch_add(1, Ordering::SeqCst);
        Ok(self
            .abordagens
            .lock()
            .expect("fixture lock")
            .iter()
            .filter(|row| ids.contains(&row.nucleo_id()))
            .cloned()
            .collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        let mut abordagens = self.abordagens.lock().expect("fixture lock");
        let index = abordagens
            .iter()
            .position(|row| row.id() == id)
            .ok_or_else(|| NucleoException::new(ReasonCode::AbordagemNotFound))?;
        abordagens.remove(index);
        self.effects.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}
