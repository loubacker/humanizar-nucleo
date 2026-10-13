use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbordagemRegisterDto {
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub nucleo_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct AbordagemUpdateDto {
    pub nome: Option<String>,
    pub descricao: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AbordagemRetrieveDto {
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub nucleo_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
