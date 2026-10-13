use chrono::NaiveDateTime;
use uuid::Uuid;
#[derive(Clone)]
pub struct NucleoRetrieveCommand {
    pub event_id: Uuid,
    pub correlation_id: Uuid,
    pub source_event_id: Uuid,
    pub nucleo_id: Uuid,
    pub callback_exchange: String,
    pub actor_id: Option<Uuid>,
    pub user_agent: Option<String>,
    pub origin_ip: Option<String>,
    pub occurred_at: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingOutcome {
    Processed,
    Duplicate,
}
