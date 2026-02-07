use crate::{
    application::dtos::{
        CreateQuestaoDto, FiltrosQuestaoDto, QuestaoListagemDto, QuestaoResponseDto,
        UpdateQuestaoDto,
    },
    domain::errors::AppResult,
    infrastructure::repositories::questao_repository::QuestaoRepository,
};
use std::sync::Arc;

pub struct QuestaoService {
    repository: Arc<QuestaoRepository>,
}

impl QuestaoService {
    pub fn new(repository: Arc<QuestaoRepository>) -> Self {
        Self { repository }
    }

    pub async fn criar_questao(&self, dto: CreateQuestaoDto) -> AppResult<QuestaoResponseDto> {
        // Validação da resposta correta
        if !["A", "B", "C", "D", "E"].contains(&dto.resposta_correta.as_str()) {
            return Err(crate::domain::errors::AppError::ValidationError(
                "Resposta correta deve ser A, B, C, D ou E".to_string(),
            ));
        }

        self.repository.criar(dto).await
    }

    pub async fn obter_questao(&self, id: i32) -> AppResult<QuestaoResponseDto> {
        self.repository.buscar_por_id(id).await
    }

    pub async fn listar_questoes(
        &self,
        skip: i64,
        limit: i64,
    ) -> AppResult<Vec<QuestaoListagemDto>> {
        self.repository.listar_todas(skip, limit).await
    }

    pub async fn buscar_questoes(
        &self,
        filtros: FiltrosQuestaoDto,
    ) -> AppResult<Vec<QuestaoListagemDto>> {
        let skip = filtros.skip.unwrap_or(0);
        let limit = filtros.limit.unwrap_or(100);

        self.repository
            .buscar_por_filtros(
                filtros.banca_id,
                filtros.disciplina,
                filtros.dificuldade,
                skip,
                limit,
            )
            .await
    }

    pub async fn atualizar_questao(
        &self,
        id: i32,
        dto: UpdateQuestaoDto,
    ) -> AppResult<QuestaoResponseDto> {
        // Validar resposta se fornecida
        if let Some(ref resposta) = dto.resposta_correta {
            if !["A", "B", "C", "D", "E"].contains(&resposta.as_str()) {
                return Err(crate::domain::errors::AppError::ValidationError(
                    "Resposta correta deve ser A, B, C, D ou E".to_string(),
                ));
            }
        }

        self.repository.atualizar(id, dto).await
    }

    pub async fn deletar_questao(&self, id: i32) -> AppResult<()> {
        self.repository.deletar(id).await
    }
}
