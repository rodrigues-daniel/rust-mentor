use crate::{
    application::dtos::{
        CreateQuestaoDto, QuestaoListagemDto, QuestaoResponseDto, UpdateQuestaoDto,
    },
    domain::errors::{AppError, AppResult},
};
use sqlx::PgPool;

pub struct QuestaoRepository {
    pool: PgPool,
}

impl QuestaoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn criar(&self, dto: CreateQuestaoDto) -> AppResult<QuestaoResponseDto> {
        let questao = sqlx::query_as::<_, QuestaoResponseDto>(
            r#"
            INSERT INTO questoes 
            (enunciado, alternativa_a, alternativa_b, alternativa_c, alternativa_d, 
             alternativa_e, resposta_correta, explicacao, dificuldade, banca_id, 
             disciplina, assunto, ano)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, enunciado, alternativa_a, alternativa_b, alternativa_c, 
                      alternativa_d, alternativa_e, resposta_correta, explicacao, 
                      dificuldade, banca_id, NULL as banca_nome, disciplina, assunto, 
                      ano, criado_em
            "#,
        )
        .bind(&dto.enunciado)
        .bind(&dto.alternativa_a)
        .bind(&dto.alternativa_b)
        .bind(&dto.alternativa_c)
        .bind(&dto.alternativa_d)
        .bind(&dto.alternativa_e)
        .bind(&dto.resposta_correta)
        .bind(&dto.explicacao)
        .bind(&dto.dificuldade)
        .bind(dto.banca_id)
        .bind(&dto.disciplina)
        .bind(&dto.assunto)
        .bind(dto.ano)
        .fetch_one(&self.pool)
        .await?;

        Ok(questao)
    }

    pub async fn buscar_por_id(&self, id: i32) -> AppResult<QuestaoResponseDto> {
        let questao = sqlx::query_as::<_, QuestaoResponseDto>(
            r#"
            SELECT q.id, q.enunciado, q.alternativa_a, q.alternativa_b, q.alternativa_c,
                   q.alternativa_d, q.alternativa_e, q.resposta_correta, q.explicacao,
                   q.dificuldade, q.banca_id, b.nome as banca_nome, q.disciplina,
                   q.assunto, q.ano, q.criado_em
            FROM questoes q
            LEFT JOIN bancas b ON q.banca_id = b.id
            WHERE q.id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Questão com id {} não encontrada", id)))?;

        Ok(questao)
    }

    pub async fn listar_todas(&self, skip: i64, limit: i64) -> AppResult<Vec<QuestaoListagemDto>> {
        let questoes = sqlx::query_as::<_, QuestaoListagemDto>(
            r#"
            SELECT q.id, q.enunciado, q.alternativa_a, q.alternativa_b, q.alternativa_c,
                   q.alternativa_d, q.alternativa_e, q.dificuldade, b.nome as banca_nome,
                   q.disciplina, q.assunto, q.ano
            FROM questoes q
            INNER JOIN bancas b ON q.banca_id = b.id
            ORDER BY q.criado_em DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(skip)
        .fetch_all(&self.pool)
        .await?;

        Ok(questoes)
    }

    pub async fn buscar_por_filtros(
        &self,
        banca_id: Option<i32>,
        disciplina: Option<String>,
        dificuldade: Option<String>,
        skip: i64,
        limit: i64,
    ) -> AppResult<Vec<QuestaoListagemDto>> {
        let mut query = String::from(
            r#"
            SELECT q.id, q.enunciado, q.alternativa_a, q.alternativa_b, q.alternativa_c,
                   q.alternativa_d, q.alternativa_e, q.dificuldade, b.nome as banca_nome,
                   q.disciplina, q.assunto, q.ano
            FROM questoes q
            INNER JOIN bancas b ON q.banca_id = b.id
            WHERE 1=1
            "#,
        );

        if banca_id.is_some() {
            query.push_str(" AND q.banca_id = $1");
        }
        if disciplina.is_some() {
            query.push_str(" AND q.disciplina = $2");
        }
        if dificuldade.is_some() {
            query.push_str(" AND q.dificuldade = $3");
        }

        query.push_str(" ORDER BY q.criado_em DESC LIMIT $4 OFFSET $5");

        let mut sql_query = sqlx::query_as::<_, QuestaoListagemDto>(&query);

        if let Some(b_id) = banca_id {
            sql_query = sql_query.bind(b_id);
        }
        if let Some(disc) = disciplina {
            sql_query = sql_query.bind(disc);
        }
        if let Some(dif) = dificuldade {
            sql_query = sql_query.bind(dif);
        }

        sql_query = sql_query.bind(limit).bind(skip);

        let questoes = sql_query.fetch_all(&self.pool).await?;

        Ok(questoes)
    }

    pub async fn atualizar(&self, id: i32, dto: UpdateQuestaoDto) -> AppResult<QuestaoResponseDto> {
        // Buscar questão existente
        let existente = self.buscar_por_id(id).await?;

        let questao = sqlx::query_as::<_, QuestaoResponseDto>(
            r#"
            UPDATE questoes
            SET enunciado = $1, alternativa_a = $2, alternativa_b = $3,
                alternativa_c = $4, alternativa_d = $5, alternativa_e = $6,
                resposta_correta = $7, explicacao = $8, dificuldade = $9,
                banca_id = $10, disciplina = $11, assunto = $12, ano = $13
            WHERE id = $14
            RETURNING id, enunciado, alternativa_a, alternativa_b, alternativa_c,
                      alternativa_d, alternativa_e, resposta_correta, explicacao,
                      dificuldade, banca_id, NULL as banca_nome, disciplina,
                      assunto, ano, criado_em
            "#,
        )
        .bind(dto.enunciado.unwrap_or(existente.enunciado))
        .bind(dto.alternativa_a.unwrap_or(existente.alternativa_a))
        .bind(dto.alternativa_b.unwrap_or(existente.alternativa_b))
        .bind(dto.alternativa_c.unwrap_or(existente.alternativa_c))
        .bind(dto.alternativa_d.unwrap_or(existente.alternativa_d))
        .bind(dto.alternativa_e.or(existente.alternativa_e))
        .bind(dto.resposta_correta.unwrap_or(existente.resposta_correta))
        .bind(dto.explicacao.or(existente.explicacao))
        .bind(dto.dificuldade.unwrap_or(existente.dificuldade))
        .bind(dto.banca_id.unwrap_or(existente.banca_id))
        .bind(dto.disciplina.unwrap_or(existente.disciplina))
        .bind(dto.assunto.unwrap_or(existente.assunto))
        .bind(dto.ano.or(existente.ano))
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(questao)
    }

    pub async fn deletar(&self, id: i32) -> AppResult<()> {
        let result = sqlx::query("DELETE FROM questoes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Questão com id {} não encontrada",
                id
            )));
        }

        Ok(())
    }
}
