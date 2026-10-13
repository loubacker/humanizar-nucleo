use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use crate::infrastructure::config::DatabaseConfig;
use crate::infrastructure::handler::database_error;
use crate::infrastructure::persistence::entity::NucleoEntity;
use uuid::Uuid;

pub(crate) struct NucleoRepository {
    database: DatabaseConfig,
}

impl NucleoRepository {
    pub fn new(database: DatabaseConfig) -> Self {
        Self { database }
    }

    pub async fn create(&self, entity: NucleoEntity) -> Result<NucleoEntity, NucleoException> {
        let client = self.database.acquire().await?;
        let row = client.query_one(
            "INSERT INTO nucleo (id, nome, descricao, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING id, nome, descricao, created_at, updated_at",
            &[&entity.id, &entity.nome, &entity.descricao, &entity.created_at, &entity.updated_at]).await.map_err(database_error)?;
        NucleoEntity::from_row(&row).map_err(database_error)
    }

    pub async fn update(&self, entity: NucleoEntity) -> Result<NucleoEntity, NucleoException> {
        let client = self.database.acquire().await?;
        let row = client.query_opt(
            "UPDATE nucleo SET nome = $2, descricao = $3, updated_at = $4 WHERE id = $1 RETURNING id, nome, descricao, created_at, updated_at",
            &[&entity.id, &entity.nome, &entity.descricao, &entity.updated_at]).await.map_err(database_error)?
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoNotFound))?;
        NucleoEntity::from_row(&row).map_err(database_error)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<NucleoEntity>, NucleoException> {
        let client = self.database.acquire().await?;
        client
            .query_opt(
                "SELECT id, nome, descricao, created_at, updated_at FROM nucleo WHERE id = $1",
                &[&id],
            )
            .await
            .map_err(database_error)?
            .as_ref()
            .map(NucleoEntity::from_row)
            .transpose()
            .map_err(database_error)
    }

    pub async fn find_all(&self) -> Result<Vec<NucleoEntity>, NucleoException> {
        let client = self.database.acquire().await?;
        client
            .query(
                "SELECT id, nome, descricao, created_at, updated_at FROM nucleo ORDER BY nome, id",
                &[],
            )
            .await
            .map_err(database_error)?
            .iter()
            .map(NucleoEntity::from_row)
            .collect::<Result<_, _>>()
            .map_err(database_error)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        let mut client = self.database.acquire().await?;
        let transaction = client.transaction().await.map_err(database_error)?;
        if transaction
            .query_opt("SELECT id FROM nucleo WHERE id = $1 FOR UPDATE", &[&id])
            .await
            .map_err(database_error)?
            .is_none()
        {
            return Err(NucleoException::new(ReasonCode::NucleoNotFound));
        }
        if transaction
            .query_opt(
                "SELECT id FROM abordagem WHERE nucleo_id = $1 LIMIT 1",
                &[&id],
            )
            .await
            .map_err(database_error)?
            .is_some()
        {
            return Err(NucleoException::new(ReasonCode::NucleoHasAbordagens));
        }
        transaction
            .execute("DELETE FROM nucleo WHERE id = $1", &[&id])
            .await
            .map_err(database_error)?;
        transaction.commit().await.map_err(database_error)
    }
}
