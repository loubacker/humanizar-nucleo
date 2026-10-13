use crate::application::http::dto::{
    AbordagemRegisterDto, AbordagemRetrieveDto, AbordagemUpdateDto,
};
use crate::infrastructure::controller::dto::SuccessResponseDto;
use crate::infrastructure::controller::extractor::{ApiJson, ApiPath};
use crate::infrastructure::controller::handler::HttpError;
use crate::infrastructure::controller::{ApplicationState, RequestContext};
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use uuid::Uuid;

pub(crate) async fn find_by_id(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<SuccessResponseDto<AbordagemRetrieveDto>, HttpError> {
    let data = state
        .abordagens
        .find_by_id(id)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "ABORDAGEM_RETRIEVED",
        context.correlation_id.to_string(),
        "Consulta realizada com sucesso.",
        data,
    ))
}

pub(crate) async fn create(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiJson(dto): ApiJson<AbordagemRegisterDto>,
) -> Result<(StatusCode, SuccessResponseDto<AbordagemRetrieveDto>), HttpError> {
    let data = state
        .abordagens
        .create(dto)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok((
        StatusCode::CREATED,
        SuccessResponseDto::new(
            "ABORDAGEM_CREATED",
            context.correlation_id.to_string(),
            "Registro criado com sucesso.",
            data,
        ),
    ))
}

pub(crate) async fn update(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiPath(id): ApiPath<Uuid>,
    ApiJson(dto): ApiJson<AbordagemUpdateDto>,
) -> Result<SuccessResponseDto<AbordagemRetrieveDto>, HttpError> {
    let data = state
        .abordagens
        .update(id, dto)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "ABORDAGEM_UPDATED",
        context.correlation_id.to_string(),
        "Registro atualizado com sucesso.",
        data,
    ))
}

pub(crate) async fn delete(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<SuccessResponseDto<()>, HttpError> {
    state
        .abordagens
        .delete(id)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "ABORDAGEM_DELETED",
        context.correlation_id.to_string(),
        "Registro excluido com sucesso.",
        (),
    ))
}
