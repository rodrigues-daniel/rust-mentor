use crate::{
    application::services::{
        banca_service::BancaService, questao_service::QuestaoService,
        treinamento_service::TreinamentoService,
    },
    presentation::handlers::{banca_handler, questao_handler, treinamento_handler},
};
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use std::sync::Arc;

pub fn criar_rotas(
    banca_service: Arc<BancaService>,
    questao_service: Arc<QuestaoService>,
    treinamento_service: Arc<TreinamentoService>,
) -> Router {
    let bancas_routes = Router::new()
        .route("/", post(banca_handler::criar_banca))
        .route("/", get(banca_handler::listar_bancas))
        .route("/:id", get(banca_handler::obter_banca))
        .route("/:id", put(banca_handler::atualizar_banca))
        .route("/:id", delete(banca_handler::deletar_banca))
        .with_state(banca_service);

    let questoes_routes = Router::new()
        .route("/", post(questao_handler::criar_questao))
        .route("/", get(questao_handler::listar_questoes))
        .route("/buscar", get(questao_handler::buscar_questoes))
        .route("/:id", get(questao_handler::obter_questao))
        .route("/:id", put(questao_handler::atualizar_questao))
        .route("/:id", delete(questao_handler::deletar_questao))
        .with_state(questao_service);

    let treinamento_routes = Router::new()
        .route("/responder", post(treinamento_handler::responder_questao))
        .route(
            "/estatisticas",
            get(treinamento_handler::obter_estatisticas),
        )
        .route("/historico", get(treinamento_handler::obter_historico))
        .route("/historico", delete(treinamento_handler::limpar_historico))
        .with_state(treinamento_service);

    Router::new()
        .nest("/api/bancas", bancas_routes)
        .nest("/api/questoes", questoes_routes)
        .nest("/api/treinamento", treinamento_routes)
}
