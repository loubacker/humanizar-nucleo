use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nucleo {
    id: Uuid,
    nome: String,
    descricao: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

impl Nucleo {
    pub fn create(nome: String, descricao: String) -> Self {
        let now = chrono::Local::now().naive_local();
        Self::restore(Uuid::new_v4(), nome, descricao, Some(now), Some(now))
    }

    pub fn restore(
        id: Uuid,
        nome: String,
        descricao: String,
        created_at: Option<NaiveDateTime>,
        updated_at: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            nome,
            descricao,
            created_at,
            updated_at,
        }
    }

    pub fn updated(&self, nome: String, descricao: String) -> Self {
        Self::restore(
            self.id,
            nome,
            descricao,
            self.created_at,
            Some(chrono::Local::now().naive_local()),
        )
    }

    pub const fn id(&self) -> Uuid {
        self.id
    }

    pub fn nome(&self) -> &str {
        &self.nome
    }

    pub fn descricao(&self) -> &str {
        &self.descricao
    }

    pub const fn created_at(&self) -> Option<NaiveDateTime> {
        self.created_at
    }

    pub const fn updated_at(&self) -> Option<NaiveDateTime> {
        self.updated_at
    }
}
