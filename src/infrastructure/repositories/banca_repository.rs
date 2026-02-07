use crate::{
    application::dtos::{CreateBancaDto, UpdateBancaDto},
    domain::{
        entities::Banca,
        errors::{AppError, AppResult},
    },
};
use sqlx::PgPool;

pub struct BancaRepository {
    pool: PgPool,
}

impl BancaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn criar(&self, dto: CreateBancaDto) -> AppResult<Banca> {
        let banca = sqlx::query_as::<_, Banca>(
            r#"
            INSERT INTO bancas (nome, sigla, descricao)
            VALUES ($1, $2, $3)
            RETURNING id, nome, sigla, descricao, criado_em
            "#,
        )
        .bind(&dto.nome)
        .bind(&dto.sigla)
        .bind(&dto.descricao)
        .fetch_one(&self.pool)
        .await?;

        Ok(banca)
    }

    pub async fn buscar_por_id(&self, id: i32) -> AppResult<Banca> {
        let banca = sqlx::query_as::<_, Banca>(
            r#"
            SELECT id, nome, sigla, descricao, criado_em
            FROM bancas
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Banca com id {} não encontrada", id)))?;

        Ok(banca)
    }

    pub async fn listar_todas(&self) -> AppResult<Vec<Banca>> {
        let bancas = sqlx::query_as::<_, Banca>(
            r#"
            SELECT id, nome, sigla, descricao, criado_em
            FROM bancas
            ORDER BY nome
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(bancas)
    }

    pub async fn atualizar(&self, id: i32, dto: UpdateBancaDto) -> AppResult<Banca> {
        // Buscar banca existente
        let banca_existente = self.buscar_por_id(id).await?;

        let banca = sqlx::query_as::<_, Banca>(
            r#"
            UPDATE bancas
            SET nome = $1, sigla = $2, descricao = $3
            WHERE id = $4
            RETURNING id, nome, sigla, descricao, criado_em
            "#,
        )
        .bind(dto.nome.unwrap_or(banca_existente.nome))
        .bind(dto.sigla.unwrap_or(banca_existente.sigla))
        .bind(dto.descricao.or(banca_existente.descricao))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(banca)
    }

    pub async fn deletar(&self, id: i32) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM bancas WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Banca com id {} não encontrada",
                id
            )));
        }

        Ok(())
    }
}
