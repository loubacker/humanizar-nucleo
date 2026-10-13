use crate::application::http::dto::{
    AbordagemRegisterDto, AbordagemRetrieveDto, AbordagemUpdateDto,
};
use crate::application::http::mapper::AbordagemMapper;
use crate::application::validator::required;
use crate::domain::exception::NucleoException;
use crate::domain::model::enums::ReasonCode;
use crate::domain::port::{AbordagemPort, NucleoPort};
use std::sync::Arc;
use uuid::Uuid;

pub struct AbordagemService {
    abordagens: Arc<dyn AbordagemPort>,
    nucleos: Arc<dyn NucleoPort>,
}

impl AbordagemService {
    pub fn new(abordagens: Arc<dyn AbordagemPort>, nucleos: Arc<dyn NucleoPort>) -> Self {
        Self {
            abordagens,
            nucleos,
        }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<AbordagemRetrieveDto, NucleoException> {
        let abordagem = self
            .abordagens
            .find_by_id(id)
            .await?
            .ok_or_else(|| NucleoException::new(ReasonCode::AbordagemNotFound))?;
        Ok(AbordagemMapper::to_dto(&abordagem))
    }

    pub async fn create(
        &self,
        dto: AbordagemRegisterDto,
    ) -> Result<AbordagemRetrieveDto, NucleoException> {
        let nome = required(dto.nome, ReasonCode::NomeRequired)?;
        let descricao = required(dto.descricao, ReasonCode::DescricaoRequired)?;
        let nucleo_id = dto
            .nucleo_id
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoIdRequired))?;
        if self.nucleos.find_by_id(nucleo_id).await?.is_none() {
            return Err(NucleoException::new(ReasonCode::NucleoNotFound));
        }
        let abordagem = self
            .abordagens
            .create(AbordagemMapper::to_created(nome, descricao, nucleo_id))
            .await?;
        Ok(AbordagemMapper::to_dto(&abordagem))
    }

    pub async fn update(
        &self,
        id: Uuid,
        dto: AbordagemUpdateDto,
    ) -> Result<AbordagemRetrieveDto, NucleoException> {
        let nome = required(dto.nome, ReasonCode::NomeRequired)?;
        let descricao = required(dto.descricao, ReasonCode::DescricaoRequired)?;
        let current = self
            .abordagens
            .find_by_id(id)
            .await?
            .ok_or_else(|| NucleoException::new(ReasonCode::AbordagemNotFound))?;
        let abordagem = self
            .abordagens
            .update(AbordagemMapper::to_updated(&current, nome, descricao))
            .await?;
        Ok(AbordagemMapper::to_dto(&abordagem))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        self.abordagens.delete(id).await
    }
}
