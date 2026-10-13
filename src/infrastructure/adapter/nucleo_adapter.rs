use crate::domain::exception::NucleoException;
use crate::domain::model::Nucleo;
use crate::domain::port::NucleoPort;
use crate::infrastructure::persistence::entity::NucleoEntity;
use crate::infrastructure::persistence::repository::NucleoRepository;
use async_trait::async_trait;
use uuid::Uuid;

pub(crate) struct NucleoAdapter {
    repository: NucleoRepository,
}

impl NucleoAdapter {
    pub fn new(repository: NucleoRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl NucleoPort for NucleoAdapter {
    async fn create(&self, model: Nucleo) -> Result<Nucleo, NucleoException> {
        Ok(to_domain(self.repository.create(to_entity(model)).await?))
    }

    async fn update(&self, model: Nucleo) -> Result<Nucleo, NucleoException> {
        Ok(to_domain(self.repository.update(to_entity(model)).await?))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Nucleo>, NucleoException> {
        Ok(self.repository.find_by_id(id).await?.map(to_domain))
    }

    async fn find_all(&self) -> Result<Vec<Nucleo>, NucleoException> {
        Ok(self
            .repository
            .find_all()
            .await?
            .into_iter()
            .map(to_domain)
            .collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        self.repository.delete(id).await
    }
}

fn to_entity(model: Nucleo) -> NucleoEntity {
    NucleoEntity {
        id: model.id(),
        nome: model.nome().to_owned(),
        descricao: model.descricao().to_owned(),
        created_at: model.created_at(),
        updated_at: model.updated_at(),
    }
}

fn to_domain(entity: NucleoEntity) -> Nucleo {
    Nucleo::restore(
        entity.id,
        entity.nome,
        entity.descricao,
        entity.created_at,
        entity.updated_at,
    )
}
