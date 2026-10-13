use crate::application::http::dto::AbordagemRetrieveDto;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct NucleoRegisterDto {
    pub nome: Option<String>,
    pub descricao: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NucleoUpdateDto {
    pub nome: Option<String>,
    pub descricao: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NucleoRetrieveDto {
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub abordagens: Vec<AbordagemRetrieveDto>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
