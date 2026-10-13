use crate::application::http::dto::{NucleoRegisterDto, NucleoRetrieveDto, NucleoUpdateDto};
use crate::application::http::mapper::NucleoMapper;
use crate::application::validator::required;
use crate::domain::exception::NucleoException;
use crate::domain::model::Abordagem;
use crate::domain::model::enums::ReasonCode;
use crate::domain::port::{AbordagemPort, NucleoPort};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct NucleoService {
    nucleos: Arc<dyn NucleoPort>,
    abordagens: Arc<dyn AbordagemPort>,
}

impl NucleoService {
    pub fn new(nucleos: Arc<dyn NucleoPort>, abordagens: Arc<dyn AbordagemPort>) -> Self {
        Self {
            nucleos,
            abordagens,
        }
    }

    pub async fn find_all(
        &self,
        with_abordagens: bool,
    ) -> Result<Vec<NucleoRetrieveDto>, NucleoException> {
        let nucleos = self.nucleos.find_all().await?;
        let mut children: HashMap<Uuid, Vec<Abordagem>> = HashMap::new();
        if with_abordagens && !nucleos.is_empty() {
            let ids: Vec<Uuid> = nucleos.iter().map(|nucleo| nucleo.id()).collect();
            for abordagem in self.abordagens.find_by_nucleo_ids(&ids).await? {
                children
                    .entry(abordagem.nucleo_id())
                    .or_default()
                    .push(abordagem);
            }
        }
        Ok(nucleos
            .iter()
            .map(|nucleo| {
                NucleoMapper::to_dto(
                    nucleo,
                    children.get(&nucleo.id()).map(Vec::as_slice).unwrap_or(&[]),
                )
            })
            .collect())
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<NucleoRetrieveDto, NucleoException> {
        let nucleo = self
            .nucleos
            .find_by_id(id)
            .await?
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoNotFound))?;
        let abordagens = self.abordagens.find_by_nucleo_ids(&[id]).await?;
        Ok(NucleoMapper::to_dto(&nucleo, &abordagens))
    }

    pub async fn create(
        &self,
        dto: NucleoRegisterDto,
    ) -> Result<NucleoRetrieveDto, NucleoException> {
        let nome = required(dto.nome, ReasonCode::NomeRequired)?;
        let descricao = required(dto.descricao, ReasonCode::DescricaoRequired)?;
        let nucleo = self
            .nucleos
            .create(NucleoMapper::to_created(nome, descricao))
            .await?;
        Ok(NucleoMapper::to_dto(&nucleo, &[]))
    }

    pub async fn update(
        &self,
        id: Uuid,
        dto: NucleoUpdateDto,
    ) -> Result<NucleoRetrieveDto, NucleoException> {
        let nome = required(dto.nome, ReasonCode::NomeRequired)?;
        let descricao = required(dto.descricao, ReasonCode::DescricaoRequired)?;
        let current = self
            .nucleos
            .find_by_id(id)
            .await?
            .ok_or_else(|| NucleoException::new(ReasonCode::NucleoNotFound))?;
        let nucleo = self
            .nucleos
            .update(NucleoMapper::to_updated(&current, nome, descricao))
            .await?;
        let abordagens = self.abordagens.find_by_nucleo_ids(&[id]).await?;
        Ok(NucleoMapper::to_dto(&nucleo, &abordagens))
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), NucleoException> {
        self.nucleos.delete(id).await
    }
}
