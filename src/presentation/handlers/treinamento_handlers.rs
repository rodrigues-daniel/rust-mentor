use crate::{
    application::{dtos::RespostaUsuarioDto, services::treinamento_service::TreinamentoService},
    domain::errors::AppResult,
};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn responder_questao(
    State(service): State<Arc<TreinamentoService>>,
    Json(dto): Json<RespostaUsuarioDto>,
) -> AppResult<Json<crate::application::dtos::ResultadoRespostaDto>> {
    let resultado = service.responder_questao(dto).await?;
    Ok(Json(resultado))
}

pub async fn obter_estatisticas(
    State(service): State<Arc<TreinamentoService>>,
) -> AppResult<Json<crate::application::dtos::EstatisticasDto>> {
    let estatisticas = service.obter_estatisticas().await?;
    Ok(Json(estatisticas))
}

pub async fn obter_historico(
    State(service): State<Arc<TreinamentoService>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> AppResult<Json<Vec<crate::application::dtos::HistoricoRespostaDto>>> {
    let skip = params
        .get("skip")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let limit = params
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(100);

    let historico = service.obter_historico(skip, limit).await?;
    Ok(Json(historico))
}

pub async fn limpar_historico(
    State(service): State<Arc<TreinamentoService>>,
) -> AppResult<StatusCode> {
    service.limpar_historico().await?;
    Ok(StatusCode::NO_CONTENT)
}
