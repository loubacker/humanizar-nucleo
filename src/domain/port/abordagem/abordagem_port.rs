use crate::domain::exception::NucleoException;
use crate::domain::model::Abordagem;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait AbordagemPort: Send + Sync {
    async fn create(&self, abordagem: Abordagem) -> Result<Abordagem, NucleoException>;
    async fn update(&self, abordagem: Abordagem) -> Result<Abordagem, NucleoException>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Abordagem>, NucleoException>;
    async fn find_by_nucleo_ids(&self, ids: &[Uuid]) -> Result<Vec<Abordagem>, NucleoException>;
    async fn delete(&self, id: Uuid) -> Result<(), NucleoException>;
}
