mod application;
mod config;
mod domain;
mod infrastructure;
mod presentation;

use crate::{
    application::services::{
        banca_service::BancaService, questao_service::QuestaoService,
        treinamento_service::TreinamentoService,
    },
    config::database::{criar_pool, executar_migrations},
    infrastructure::repositories::{
        banca_repository::BancaRepository, historico_repository::HistoricoRepository,
        questao_repository::QuestaoRepository,
    },
    presentation::routes::criar_rotas,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carregar variáveis de ambiente
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost/concursos".to_string());

    // Criar pool de conexões
    let pool = criar_pool(&database_url).await?;
    println!("✅ Conectado ao banco de dados");

    // Executar migrations
    executar_migrations(&pool).await?;
    println!("✅ Migrations executadas");

    // Criar repositórios
    let banca_repo = Arc::new(BancaRepository::new(pool.clone()));
    let questao_repo = Arc::new(QuestaoRepository::new(pool.clone()));
    let historico_repo = Arc::new(HistoricoRepository::new(pool.clone()));

    // Criar serviços
    let banca_service = Arc::new(BancaService::new(banca_repo));
    let questao_service = Arc::new(QuestaoService::new(questao_repo.clone()));
    let treinamento_service = Arc::new(TreinamentoService::new(questao_repo, historico_repo));

    // Criar rotas
    let app = criar_rotas(banca_service, questao_service, treinamento_service);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Servidor rodando em http://0.0.0.0:3000");

    axum::serve(listener, app).await?;

    Ok(())
}
