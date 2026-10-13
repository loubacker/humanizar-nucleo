use chrono::NaiveDateTime;
use tokio_postgres::Row;
use uuid::Uuid;
pub(crate) struct NucleoEntity {
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NucleoEntity {
    pub fn from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            nome: row.try_get("nome")?,
            descricao: row.try_get("descricao")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
