use chrono::NaiveDateTime;
use tokio_postgres::Row;
use uuid::Uuid;
pub(crate) struct AbordagemEntity {
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub nucleo_id: Uuid,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl AbordagemEntity {
    pub fn from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            nome: row.try_get("nome")?,
            descricao: row.try_get("descricao")?,
            nucleo_id: row.try_get("nucleo_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
