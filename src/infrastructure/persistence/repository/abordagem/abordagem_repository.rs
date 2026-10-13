use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::handler::{abordagem_create_error, database_error};
use crate::infrastructure::persistence::entity::AbordagemEntity;
use uuid::Uuid;

pub(crate) struct AbordagemRepository {
    database: DatabaseConfig,
}

impl AbordagemRepository {
    pub fn new(database: DatabaseConfig) -> Self {
        Self { database }
    }

    pub async fn create(
        &self,
        entity: AbordagemEntity,
    ) -> Result<AbordagemEntity, NucleoException> {
        let client = self.database.acquire().await?;
        let row = client.query_one(
            "INSERT INTO abordagem (id, nome, descricao, nucleo_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, nome, descricao, nucleo_id, created_at, updated_at",
            &[&entity.id, &entity.nome, &entity.descricao, &entity.nucleo_id, &entity.created_at, &entity.updated_at]).await.map_err(abordagem_create_error)?;
        AbordagemEntity::from_row(&row).map_err(database_error)
    }

    pub async fn update(
        &self,
        entity: AbordagemEntity,
    ) -> Result<AbordagemEntity, NucleoException> {
        let client = self.database.acquire().await?;
        let row = client.query_opt(
            "UPDATE abordagem SET nome = $2, descricao = $3, updated_at = $4 WHERE id = $1 RETURNING id, nome, descricao, nucleo_id, created_at, updated_at",
            &[&entity.id, &entity.nome, &entity.descricao, &entity.updated_at]).await.map_err(database_error)?
            .ok_or_else(|| NucleoException::new(ReasonCode::AbordagemNotFound))?;
        AbordagemEntity::from_row(&row).map_err(database_error)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AbordagemEntity>, NucleoException> {
        let client = self.database.acquire().await?;
        client.query_opt("SELECT id, nome, descricao, nucleo_id, created_at, updated_at FROM abordagem WHERE id = $1", &[&id]).await.map_err(database_error)?
            .as_ref().map(AbordagemEntity::from_row).transpose().map_err(database_error)
    }

    pub async fn find_by_nucleo_ids(
        &self,
        ids: &[Uuid],
    ) -> Result<Vec<AbordagemEntity>, NucleoException> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let client = self.database.acquire().await?;
        client.query("SELECT id, nome, descricao, nucleo_id, created_at, updated_at FROM abordagem WHERE nucleo_id = ANY($1) ORDER BY nucleo_id, nome, id", &[&ids]).await.map_err(database_error)?
            .iter().map(AbordagemEntity::from_row).collect::<Result<_, _>>().map_err(database_error)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        let client = self.database.acquire().await?;
        let count = client
            .execute("DELETE FROM abordagem WHERE id = $1", &[&id])
            .await
            .map_err(database_error)?;
        if count == 0 {
            return Err(NucleoException::new(ReasonCode::AbordagemNotFound));
        }
        Ok(())
    }
}
