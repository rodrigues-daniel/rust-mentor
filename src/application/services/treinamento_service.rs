use crate::{
    application::dtos::{
        EstatisticasDto, HistoricoRespostaDto, RespostaUsuarioDto, ResultadoRespostaDto,
    },
    domain::errors::AppResult,
    infrastructure::repositories::{
        historico_repository::HistoricoRepository, questao_repository::QuestaoRepository,
    },
};
use std::sync::Arc;

pub struct TreinamentoService {
    questao_repository: Arc<QuestaoRepository>,
    historico_repository: Arc<HistoricoRepository>,
}

impl TreinamentoService {
    pub fn new(
        questao_repository: Arc<QuestaoRepository>,
        historico_repository: Arc<HistoricoRepository>,
    ) -> Self {
        Self {
            questao_repository,
            historico_repository,
        }
    }

    pub async fn responder_questao(
        &self,
        dto: RespostaUsuarioDto,
    ) -> AppResult<ResultadoRespostaDto> {
        // Buscar a questão
        let questao = self
            .questao_repository
            .buscar_por_id(dto.questao_id)
            .await?;

        // Verificar se a resposta está correta
        let esta_correto = dto.resposta_escolhida == questao.resposta_correta;

        // Registrar no histórico
        self.historico_repository
            .registrar_resposta(dto.questao_id, dto.resposta_escolhida.clone(), esta_correto)
            .await?;

        Ok(ResultadoRespostaDto {
            questao_id: questao.id,
            resposta_escolhida: dto.resposta_escolhida,
            resposta_correta: questao.resposta_correta,
            esta_correto,
            explicacao: questao.explicacao,
        })
    }

    pub async fn obter_estatisticas(&self) -> AppResult<EstatisticasDto> {
        self.historico_repository.obter_estatisticas().await
    }

    pub async fn obter_historico(
        &self,
        skip: i64,
        limit: i64,
    ) -> AppResult<Vec<HistoricoRespostaDto>> {
        self.historico_repository
            .listar_historico(skip, limit)
            .await
    }

    pub async fn limpar_historico(&self) -> AppResult<()> {
        self.historico_repository.limpar_historico().await
    }
}
