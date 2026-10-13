use crate::domain::exception::NucleoException;
use crate::domain::model::Abordagem;
use crate::domain::port::AbordagemPort;
use crate::infrastructure::persistence::entity::AbordagemEntity;
use crate::infrastructure::persistence::repository::AbordagemRepository;
use async_trait::async_trait;
use uuid::Uuid;

pub(crate) struct AbordagemAdapter {
    repository: AbordagemRepository,
}

impl AbordagemAdapter {
    pub fn new(repository: AbordagemRepository) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl AbordagemPort for AbordagemAdapter {
    async fn create(&self, model: Abordagem) -> Result<Abordagem, NucleoException> {
        Ok(to_domain(self.repository.create(to_entity(model)).await?))
    }

    async fn update(&self, model: Abordagem) -> Result<Abordagem, NucleoException> {
        Ok(to_domain(self.repository.update(to_entity(model)).await?))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Abordagem>, NucleoException> {
        Ok(self.repository.find_by_id(id).await?.map(to_domain))
    }

    async fn find_by_nucleo_ids(&self, ids: &[Uuid]) -> Result<Vec<Abordagem>, NucleoException> {
        Ok(self
            .repository
            .find_by_nucleo_ids(ids)
            .await?
            .into_iter()
            .map(to_domain)
            .collect())
    }

    async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        self.repository.delete(id).await
    }
}

fn to_entity(model: Abordagem) -> AbordagemEntity {
    AbordagemEntity {
        id: model.id(),
        nome: model.nome().to_owned(),
        descricao: model.descricao().to_owned(),
        nucleo_id: model.nucleo_id(),
        created_at: model.created_at(),
        updated_at: model.updated_at(),
    }
}

fn to_domain(entity: AbordagemEntity) -> Abordagem {
    Abordagem::restore(
        entity.id,
        entity.nome,
        entity.descricao,
        entity.nucleo_id,
        entity.created_at,
        entity.updated_at,
    )
}
