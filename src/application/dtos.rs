use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// DTOs de Request
#[derive(Debug, Deserialize)]
pub struct CreateBancaDto {
    pub nome: String,
    pub sigla: String,
    pub descricao: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBancaDto {
    pub nome: Option<String>,
    pub sigla: Option<String>,
    pub descricao: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestaoDto {
    pub enunciado: String,
    pub alternativa_a: String,
    pub alternativa_b: String,
    pub alternativa_c: String,
    pub alternativa_d: String,
    pub alternativa_e: Option<String>,
    pub resposta_correta: String,
    pub explicacao: Option<String>,
    pub dificuldade: String,
    pub banca_id: i32,
    pub disciplina: String,
    pub assunto: String,
    pub ano: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestaoDto {
    pub enunciado: Option<String>,
    pub alternativa_a: Option<String>,
    pub alternativa_b: Option<String>,
    pub alternativa_c: Option<String>,
    pub alternativa_d: Option<String>,
    pub alternativa_e: Option<String>,
    pub resposta_correta: Option<String>,
    pub explicacao: Option<String>,
    pub dificuldade: Option<String>,
    pub banca_id: Option<i32>,
    pub disciplina: Option<String>,
    pub assunto: Option<String>,
    pub ano: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RespostaUsuarioDto {
    pub questao_id: i32,
    pub resposta_escolhida: String,
}

// DTOs de Response
#[derive(Debug, Serialize)]
pub struct BancaResponseDto {
    pub id: i32,
    pub nome: String,
    pub sigla: String,
    pub descricao: Option<String>,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct QuestaoResponseDto {
    pub id: i32,
    pub enunciado: String,
    pub alternativa_a: String,
    pub alternativa_b: String,
    pub alternativa_c: String,
    pub alternativa_d: String,
    pub alternativa_e: Option<String>,
    pub resposta_correta: String,
    pub explicacao: Option<String>,
    pub dificuldade: String,
    pub banca_id: i32,
    pub banca_nome: Option<String>,
    pub disciplina: String,
    pub assunto: String,
    pub ano: Option<i32>,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct QuestaoListagemDto {
    pub id: i32,
    pub enunciado: String,
    pub alternativa_a: String,
    pub alternativa_b: String,
    pub alternativa_c: String,
    pub alternativa_d: String,
    pub alternativa_e: Option<String>,
    pub dificuldade: String,
    pub banca_nome: String,
    pub disciplina: String,
    pub assunto: String,
    pub ano: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct ResultadoRespostaDto {
    pub questao_id: i32,
    pub resposta_escolhida: String,
    pub resposta_correta: String,
    pub esta_correto: bool,
    pub explicacao: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EstatisticasDto {
    pub total_questoes_respondidas: i64,
    pub total_acertos: i64,
    pub total_erros: i64,
    pub percentual_acerto: f64,
    pub questoes_por_banca: HashMap<String, i64>,
    pub questoes_por_disciplina: HashMap<String, i64>,
    pub questoes_por_dificuldade: HashMap<String, i64>,
}

#[derive(Debug, Serialize)]
pub struct HistoricoRespostaDto {
    pub id: i32,
    pub questao_id: i32,
    pub enunciado_questao: String,
    pub resposta_escolhida: String,
    pub resposta_correta: String,
    pub esta_correto: bool,
    pub banca_nome: String,
    pub disciplina: String,
    pub respondido_em: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct FiltrosQuestaoDto {
    pub banca_id: Option<i32>,
    pub disciplina: Option<String>,
    pub dificuldade: Option<String>,
    pub skip: Option<i64>,
    pub limit: Option<i64>,
}
