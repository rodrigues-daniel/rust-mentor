use crate::{
    application::{
        dtos::{CreateBancaDto, UpdateBancaDto},
        services::banca_service::BancaService,
    },
    domain::errors::AppResult,
};
use axum::{Json, extract::Path, extract::State, http::StatusCode};
use std::sync::Arc;

pub async fn criar_banca(
    State(service): State<Arc<BancaService>>,
    Json(dto): Json<CreateBancaDto>,
) -> AppResult<(StatusCode, Json<crate::application::dtos::BancaResponseDto>)> {
    let banca = service.criar_banca(dto).await?;
    Ok((StatusCode::CREATED, Json(banca)))
}

pub async fn obter_banca(
    State(service): State<Arc<BancaService>>,
    Path(id): Path<i32>,
) -> AppResult<Json<crate::application::dtos::BancaResponseDto>> {
    let banca = service.obter_banca(id).await?;
    Ok(Json(banca))
}

pub async fn listar_bancas(
    State(service): State<Arc<BancaService>>,
) -> AppResult<Json<Vec<crate::application::dtos::BancaResponseDto>>> {
    let bancas = service.listar_bancas().await?;
    Ok(Json(bancas))
}

pub async fn atualizar_banca(
    State(service): State<Arc<BancaService>>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateBancaDto>,
) -> AppResult<Json<crate::application::dtos::BancaResponseDto>> {
    let banca = service.atualizar_banca(id, dto).await?;
    Ok(Json(banca))
}

pub async fn deletar_banca(
    State(service): State<Arc<BancaService>>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    service.deletar_banca(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
