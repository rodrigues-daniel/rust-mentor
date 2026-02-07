use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dificuldade {
    Facil,
    Medio,
    Dificil,
}

impl Dificuldade {
    pub fn as_str(&self) -> &str {
        match self {
            Dificuldade::Facil => "facil",
            Dificuldade::Medio => "medio",
            Dificuldade::Dificil => "dificil",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "facil" => Some(Dificuldade::Facil),
            "medio" => Some(Dificuldade::Medio),
            "dificil" => Some(Dificuldade::Dificil),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Banca {
    pub id: i32,
    pub nome: String,
    pub sigla: String,
    pub descricao: Option<String>,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Questao {
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
    pub disciplina: String,
    pub assunto: String,
    pub ano: Option<i32>,
    pub criado_em: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct HistoricoResposta {
    pub id: i32,
    pub questao_id: i32,
    pub resposta_escolhida: String,
    pub esta_correto: bool,
    pub respondido_em: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dificuldade_as_str_deve_retornar_string_correta() {
        assert_eq!(Dificuldade::Facil.as_str(), "facil");
        assert_eq!(Dificuldade::Medio.as_str(), "medio");
        assert_eq!(Dificuldade::Dificil.as_str(), "dificil");
    }

    #[test]
    fn dificuldade_from_str_deve_converter_corretamente() {
        assert!(matches!(
            Dificuldade::from_str("facil"),
            Some(Dificuldade::Facil)
        ));

        assert!(matches!(
            Dificuldade::from_str("medio"),
            Some(Dificuldade::Medio)
        ));

        assert!(matches!(
            Dificuldade::from_str("dificil"),
            Some(Dificuldade::Dificil)
        ));
    }

    #[test]
    fn dificuldade_from_str_deve_retornar_none_para_valores_invalidos() {
        assert!(Dificuldade::from_str("FACIL").is_none());
        assert!(Dificuldade::from_str("easy").is_none());
        assert!(Dificuldade::from_str("").is_none());
        assert!(Dificuldade::from_str("qualquer").is_none());
    }
}
