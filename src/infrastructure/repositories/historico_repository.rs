use crate::{
    application::dtos::{EstatisticasDto, HistoricoRespostaDto},
    domain::{entities::HistoricoResposta, errors::AppResult},
};
use sqlx::PgPool;
use std::collections::HashMap;

pub struct HistoricoRepository {
    pool: PgPool,
}

impl HistoricoRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn registrar_resposta(
        &self,
        questao_id: i32,
        resposta_escolhida: String,
        esta_correto: bool,
    ) -> AppResult<HistoricoResposta> {
        let historico = sqlx::query_as::<_, HistoricoResposta>(
            r#"
            INSERT INTO historico_respostas (questao_id, resposta_escolhida, esta_correto)
            VALUES ($1, $2, $3)
            RETURNING id, questao_id, resposta_escolhida, esta_correto, respondido_em
            "#,
        )
        .bind(questao_id)
        .bind(&resposta_escolhida)
        .bind(esta_correto)
        .fetch_one(&self.pool)
        .await?;

        Ok(historico)
    }

    pub async fn obter_estatisticas(&self) -> AppResult<EstatisticasDto> {
        // Total de questões respondidas
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM historico_respostas")
            .fetch_one(&self.pool)
            .await?;

        // Total de acertos
        let acertos: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM historico_respostas WHERE esta_correto = true")
                .fetch_one(&self.pool)
                .await?;

        // Total de erros
        let erros: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM historico_respostas WHERE esta_correto = false")
                .fetch_one(&self.pool)
                .await?;

        let percentual_acerto = if total.0 > 0 {
            (acertos.0 as f64 / total.0 as f64) * 100.0
        } else {
            0.0
        };

        // Questões por banca
        let bancas: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT b.nome, COUNT(hr.id)
            FROM historico_respostas hr
            JOIN questoes q ON hr.questao_id = q.id
            JOIN bancas b ON q.banca_id = b.id
            GROUP BY b.nome
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut questoes_por_banca = HashMap::new();
        for (banca, count) in bancas {
            questoes_por_banca.insert(banca, count);
        }

        // Questões por disciplina
        let disciplinas: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT q.disciplina, COUNT(hr.id)
            FROM historico_respostas hr
            JOIN questoes q ON hr.questao_id = q.id
            GROUP BY q.disciplina
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut questoes_por_disciplina = HashMap::new();
        for (disciplina, count) in disciplinas {
            questoes_por_disciplina.insert(disciplina, count);
        }

        // Questões por dificuldade
        let dificuldades: Vec<(String, i64)> = sqlx::query_as(
            r#"
            SELECT q.dificuldade, COUNT(hr.id)
            FROM historico_respostas hr
            JOIN questoes q ON hr.questao_id = q.id
            GROUP BY q.dificuldade
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let mut questoes_por_dificuldade = HashMap::new();
        for (dificuldade, count) in dificuldades {
            questoes_por_dificuldade.insert(dificuldade, count);
        }

        Ok(EstatisticasDto {
            total_questoes_respondidas: total.0,
            total_acertos: acertos.0,
            total_erros: erros.0,
            percentual_acerto,
            questoes_por_banca,
            questoes_por_disciplina,
            questoes_por_dificuldade,
        })
    }

    pub async fn listar_historico(
        &self,
        skip: i64,
        limit: i64,
    ) -> AppResult<Vec<HistoricoRespostaDto>> {
        let historico = sqlx::query_as::<_, HistoricoRespostaDto>(
            r#"
            SELECT hr.id, hr.questao_id, q.enunciado as enunciado_questao,
                   hr.resposta_escolhida, q.resposta_correta, hr.esta_correto,
                   b.nome as banca_nome, q.disciplina, hr.respondido_em
            FROM historico_respostas hr
            JOIN questoes q ON hr.questao_id = q.id
            JOIN bancas b ON q.banca_id = b.id
            ORDER BY hr.respondido_em DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(skip)
        .fetch_all(&self.pool)
        .await?;

        Ok(historico)
    }

    pub async fn limpar_historico(&self) -> AppResult<()> {
        sqlx::query("DELETE FROM historico_respostas")
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
