use crate::{
    application::{
        dtos::{CreateQuestaoDto, FiltrosQuestaoDto, UpdateQuestaoDto},
        services::questao_service::QuestaoService,
    },
    domain::errors::AppResult,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn criar_questao(
    State(service): State<Arc<QuestaoService>>,
    Json(dto): Json<CreateQuestaoDto>,
) -> AppResult<(
    StatusCode,
    Json<crate::application::dtos::QuestaoResponseDto>,
)> {
    let questao = service.criar_questao(dto).await?;
    Ok((StatusCode::CREATED, Json(questao)))
}

pub async fn obter_questao(
    State(service): State<Arc<QuestaoService>>,
    Path(id): Path<i32>,
) -> AppResult<Json<crate::application::dtos::QuestaoResponseDto>> {
    let questao = service.obter_questao(id).await?;
    Ok(Json(questao))
}

pub async fn listar_questoes(
    State(service): State<Arc<QuestaoService>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<crate::application::dtos::QuestaoListagemDto>>> {
    let skip = params
        .get("skip")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(100);

    let questoes = service.listar_questoes(skip, limit).await?;
    Ok(Json(questoes))
}

pub async fn buscar_questoes(
    State(service): State<Arc<QuestaoService>>,
    Query(filtros): Query<FiltrosQuestaoDto>,
) -> AppResult<Json<Vec<crate::application::dtos::QuestaoListagemDto>>> {
    let questoes = service.buscar_questoes(filtros).await?;
    Ok(Json(questoes))
}

pub async fn atualizar_questao(
    State(service): State<Arc<QuestaoService>>,
    Path(id): Path<i32>,
    Json(dto): Json<UpdateQuestaoDto>,
) -> AppResult<Json<crate::application::dtos::QuestaoResponseDto>> {
    let questao = service.atualizar_questao(id, dto).await?;
    Ok(Json(questao))
}

pub async fn deletar_questao(
    State(service): State<Arc<QuestaoService>>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    service.deletar_questao(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
