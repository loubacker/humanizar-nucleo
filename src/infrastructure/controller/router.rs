use super::abordagem::abordagem_controller;
use super::nucleo::nucleo_controller;
use super::{ApplicationState, http_observability};
use crate::infrastructure::config::SecurityConfig;
use axum::extract::{DefaultBodyLimit, State};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, post, put};
use axum::{Extension, Json, Router};
use std::sync::atomic::Ordering;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use url::Url;

pub fn create_router(state: ApplicationState, security: &SecurityConfig) -> Router {
    create_router_with_limits(state, security, 2 * 1024 * 1024, Duration::from_secs(30))
}

pub fn create_router_with_limits(
    state: ApplicationState,
    security: &SecurityConfig,
    body_limit: usize,
    timeout: Duration,
) -> Router {
    let health_state = state.clone();
    let reads = security.protect_authenticated(
        Router::new()
            .route("/api/v1/nucleos", get(nucleo_controller::find_all))
            .route(
                "/api/v1/nucleos/abordagens",
                get(nucleo_controller::find_all_with_abordagens),
            )
            .route("/api/v1/nucleo/{id}", get(nucleo_controller::find_by_id))
            .route("/api/v1/nucleos/{id}", get(nucleo_controller::find_by_id))
            .route(
                "/api/v1/abordagem/{id}",
                get(abordagem_controller::find_by_id),
            )
            .with_state(state.clone()),
    );
    let writes = security.protect_administrator(
        Router::new()
            .route("/api/v1/nucleo/register", post(nucleo_controller::create))
            .route("/api/v1/nucleo/update/{id}", put(nucleo_controller::update))
            .route(
                "/api/v1/nucleo/delete/{id}",
                delete(nucleo_controller::delete),
            )
            .route(
                "/api/v1/abordagem/register",
                post(abordagem_controller::create),
            )
            .route(
                "/api/v1/abordagem/update/{id}",
                put(abordagem_controller::update),
            )
            .route(
                "/api/v1/abordagem/delete/{id}",
                delete(abordagem_controller::delete),
            )
            .with_state(state),
    );
    let router = reads
        .merge(writes)
        .merge(
            Router::new()
                .route("/health", get(health))
                .with_state(health_state),
        )
        .fallback(StatusCode::NOT_FOUND)
        .layer(DefaultBodyLimit::max(body_limit));
    http_observability::apply(router, timeout, cors())
}

async fn health(
    State(state): State<ApplicationState>,
    Extension(context): Extension<super::RequestContext>,
) -> Response {
    if !state.messaging_ready.load(Ordering::Acquire) {
        return super::handler::HttpError::reason(
            crate::domain::model::enums::ReasonCode::MessagingUnavailable,
            context,
        )
        .into_response();
    }
    if let Some(database) = &state.database {
        let probe = async {
            let client = database
                .acquire()
                .await
                .map_err(crate::domain::exception::NucleoException::from)?;
            client.query_one("SELECT 1", &[]).await.map_err(|error| {
                crate::domain::exception::NucleoException::with_source(
                    crate::domain::model::enums::ReasonCode::PersistenceFailure,
                    error,
                )
            })?;
            Ok::<(), crate::domain::exception::NucleoException>(())
        };
        match tokio::time::timeout(Duration::from_secs(2), probe).await {
            Ok(Ok(())) => {}
            Ok(Err(error)) => {
                return super::handler::HttpError::new(error, context).into_response();
            }
            Err(error) => {
                return super::handler::HttpError::new(
                    crate::domain::exception::NucleoException::with_source(
                        crate::domain::model::enums::ReasonCode::PersistenceFailure,
                        error,
                    ),
                    context,
                )
                .into_response();
            }
        }
    }
    Json(serde_json::json!({"status":"UP"})).into_response()
}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(|origin: &HeaderValue, _| {
            let Some(url) = origin
                .to_str()
                .ok()
                .and_then(|value| Url::parse(value).ok())
            else {
                return false;
            };
            if url.scheme() != "http" {
                return false;
            }
            match url.host_str() {
                Some("localhost" | "127.0.0.1") => true,
                Some(host) => host
                    .parse::<std::net::Ipv4Addr>()
                    .is_ok_and(|address| address.octets()[0] == 10),
                None => false,
            }
        }))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers(
            [
                "authorization",
                "content-type",
                "accept",
                "origin",
                "x-requested-with",
                "x-correlation-id",
                "x-page-visited",
                "x-csrf-token",
            ]
            .map(axum::http::HeaderName::from_static),
        )
        .expose_headers(
            ["authorization", "x-correlation-id"].map(axum::http::HeaderName::from_static),
        )
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}
