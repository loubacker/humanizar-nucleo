use crate::application::http::dto::NucleoRetrieveDto;
use crate::application::http::mapper::AbordagemMapper;
use crate::domain::model::{Abordagem, Nucleo};

pub struct NucleoMapper;
impl NucleoMapper {
    pub fn to_dto(nucleo: &Nucleo, abordagens: &[Abordagem]) -> NucleoRetrieveDto {
        NucleoRetrieveDto {
            id: nucleo.id(),
            nome: nucleo.nome().to_owned(),
            descricao: nucleo.descricao().to_owned(),
            abordagens: abordagens.iter().map(AbordagemMapper::to_dto).collect(),
            created_at: nucleo.created_at(),
            updated_at: nucleo.updated_at(),
        }
    }

    pub fn to_created(nome: String, descricao: String) -> Nucleo {
        Nucleo::create(nome, descricao)
    }

    pub fn to_updated(current: &Nucleo, nome: String, descricao: String) -> Nucleo {
        current.updated(nome, descricao)
    }
}
