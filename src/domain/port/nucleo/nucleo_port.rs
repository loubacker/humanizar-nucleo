use crate::domain::exception::NucleoException;
use crate::domain::model::Nucleo;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait NucleoPort: Send + Sync {
    async fn create(&self, nucleo: Nucleo) -> Result<Nucleo, NucleoException>;
    async fn update(&self, nucleo: Nucleo) -> Result<Nucleo, NucleoException>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Nucleo>, NucleoException>;
    async fn find_all(&self) -> Result<Vec<Nucleo>, NucleoException>;
    async fn delete(&self, id: Uuid) -> Result<(), NucleoException>;
}
