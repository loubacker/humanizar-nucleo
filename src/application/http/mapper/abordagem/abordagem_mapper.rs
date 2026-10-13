use crate::application::http::dto::AbordagemRetrieveDto;
use crate::domain::model::Abordagem;
use uuid::Uuid;

pub struct AbordagemMapper;
impl AbordagemMapper {
    pub fn to_dto(abordagem: &Abordagem) -> AbordagemRetrieveDto {
        AbordagemRetrieveDto {
            id: abordagem.id(),
            nome: abordagem.nome().to_owned(),
            descricao: abordagem.descricao().to_owned(),
            nucleo_id: abordagem.nucleo_id(),
            created_at: abordagem.created_at(),
            updated_at: abordagem.updated_at(),
        }
    }

    pub fn to_created(nome: String, descricao: String, nucleo_id: Uuid) -> Abordagem {
        Abordagem::create(nome, descricao, nucleo_id)
    }

    pub fn to_updated(current: &Abordagem, nome: String, descricao: String) -> Abordagem {
        current.updated(nome, descricao)
    }
}
