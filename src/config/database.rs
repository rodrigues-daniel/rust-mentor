use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn criar_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn executar_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS bancas (
            id SERIAL PRIMARY KEY,
            nome VARCHAR(100) NOT NULL UNIQUE,
            sigla VARCHAR(10) NOT NULL UNIQUE,
            descricao TEXT,
            criado_em TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS questoes (
            id SERIAL PRIMARY KEY,
            enunciado TEXT NOT NULL,
            alternativa_a TEXT NOT NULL,
            alternativa_b TEXT NOT NULL,
            alternativa_c TEXT NOT NULL,
            alternativa_d TEXT NOT NULL,
            alternativa_e TEXT,
            resposta_correta VARCHAR(1) NOT NULL,
            explicacao TEXT,
            dificuldade VARCHAR(20) NOT NULL,
            banca_id INTEGER NOT NULL REFERENCES bancas(id) ON DELETE CASCADE,
            disciplina VARCHAR(100) NOT NULL,
            assunto VARCHAR(100) NOT NULL,
            ano INTEGER,
            criado_em TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS historico_respostas (
            id SERIAL PRIMARY KEY,
            questao_id INTEGER NOT NULL REFERENCES questoes(id) ON DELETE CASCADE,
            resposta_escolhida VARCHAR(1) NOT NULL,
            esta_correto BOOLEAN NOT NULL,
            respondido_em TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
