use crate::application::http::dto::{NucleoRegisterDto, NucleoRetrieveDto, NucleoUpdateDto};
use crate::infrastructure::controller::dto::SuccessResponseDto;
use crate::infrastructure::controller::extractor::{ApiJson, ApiPath};
use crate::infrastructure::controller::handler::HttpError;
use crate::infrastructure::controller::{ApplicationState, RequestContext};
use axum::Extension;
use axum::extract::State;
use axum::http::StatusCode;
use uuid::Uuid;

pub(crate) async fn find_all(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
) -> Result<SuccessResponseDto<Vec<NucleoRetrieveDto>>, HttpError> {
    let data = state
        .nucleos
        .find_all(false)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "NUCLEOS_RETRIEVED",
        context.correlation_id.to_string(),
        "Consulta realizada com sucesso.",
        data,
    ))
}

pub(crate) async fn find_all_with_abordagens(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
) -> Result<SuccessResponseDto<Vec<NucleoRetrieveDto>>, HttpError> {
    let data = state
        .nucleos
        .find_all(true)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "NUCLEOS_WITH_ABORDAGENS_RETRIEVED",
        context.correlation_id.to_string(),
        "Consulta realizada com sucesso.",
        data,
    ))
}

pub(crate) async fn find_by_id(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiPath(id): ApiPath<Uuid>,
) -> Result<SuccessResponseDto<NucleoRetrieveDto>, HttpError> {
    let data = state
        .nucleos
        .find_by_id(id)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "NUCLEO_RETRIEVED",
        context.correlation_id.to_string(),
        "Consulta realizada com sucesso.",
        data,
    ))
}

pub(crate) async fn create(
    State(state): State<ApplicationState>,
    Extension(context): Extension<RequestContext>,
    ApiJson(dto): ApiJson<NucleoRegisterDto>,
) -> Result<(StatusCode, SuccessResponseDto<NucleoRetrieveDto>), HttpError> {
    let data = state
        .nucleos
        .create(dto)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok((
        StatusCode::CREATED,
        SuccessResponseDto::new(
            "NUCLEO_CREATED",
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
    ApiJson(dto): ApiJson<NucleoUpdateDto>,
) -> Result<SuccessResponseDto<NucleoRetrieveDto>, HttpError> {
    let data = state
        .nucleos
        .update(id, dto)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "NUCLEO_UPDATED",
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
        .nucleos
        .delete(id)
        .await
        .map_err(|error| HttpError::new(error, context.clone()))?;
    Ok(SuccessResponseDto::new(
        "NUCLEO_DELETED",
        context.correlation_id.to_string(),
        "Registro excluido com sucesso.",
        (),
    ))
}
