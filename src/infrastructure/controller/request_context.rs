use axum::extract::Request;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub correlation_id: Uuid,
    pub path: String,
}

impl RequestContext {
    pub(crate) fn from_request(request: &Request) -> Self {
        request
            .extensions()
            .get::<Self>()
            .cloned()
            .unwrap_or_else(|| Self {
                correlation_id: Uuid::new_v4(),
                path: request.uri().path().to_owned(),
            })
    }
}
