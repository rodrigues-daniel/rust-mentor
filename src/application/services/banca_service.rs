use crate::{
    application::dtos::{BancaResponseDto, CreateBancaDto, UpdateBancaDto},
    domain::{entities::Banca, errors::AppResult},
    infrastructure::repositories::banca_repository::BancaRepository,
};
use std::sync::Arc;

pub struct BancaService {
    repository: Arc<BancaRepository>,
}

impl BancaService {
    pub fn new(repository: Arc<BancaRepository>) -> Self {
        Self { repository }
    }

    pub async fn criar_banca(&self, dto: CreateBancaDto) -> AppResult<BancaResponseDto> {
        let banca = self.repository.criar(dto).await?;
        Ok(self.to_response_dto(banca))
    }

    pub async fn obter_banca(&self, id: i32) -> AppResult<BancaResponseDto> {
        let banca = self.repository.buscar_por_id(id).await?;
        Ok(self.to_response_dto(banca))
    }

    pub async fn listar_bancas(&self) -> AppResult<Vec<BancaResponseDto>> {
        let bancas = self.repository.listar_todas().await?;
        Ok(bancas
            .into_iter()
            .map(|b| self.to_response_dto(b))
            .collect())
    }

    pub async fn atualizar_banca(
        &self,
        id: i32,
        dto: UpdateBancaDto,
    ) -> AppResult<BancaResponseDto> {
        let banca = self.repository.atualizar(id, dto).await?;
        Ok(self.to_response_dto(banca))
    }

    pub async fn deletar_banca(&self, id: i32) -> AppResult<()> {
        self.repository.deletar(id).await
    }

    fn to_response_dto(&self, banca: Banca) -> BancaResponseDto {
        BancaResponseDto {
            id: banca.id,
            nome: banca.nome,
            sigla: banca.sigla,
            descricao: banca.descricao,
            criado_em: banca.criado_em,
        }
    }
}
