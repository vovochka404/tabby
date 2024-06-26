use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Default, Serialize, Deserialize, Debug, ToSchema)]
pub struct CodeSearchResponse {
    pub num_hits: usize,
    pub hits: Vec<CodeSearchHit>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CodeSearchHit {
    pub score: f32,
    pub doc: CodeSearchDocument,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct CodeSearchDocument {
    pub body: String,
    pub filepath: String,
    pub git_url: String,
    pub language: String,
}

#[derive(Error, Debug)]
pub enum CodeSearchError {
    #[error("index not ready")]
    NotReady,

    #[error(transparent)]
    QueryParserError(#[from] tantivy::query::QueryParserError),

    #[error(transparent)]
    TantivyError(#[from] tantivy::TantivyError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[async_trait]
pub trait CodeSearch: Send + Sync {
    async fn search(
        &self,
        q: &str,
        limit: usize,
        offset: usize,
    ) -> Result<CodeSearchResponse, CodeSearchError>;

    async fn search_in_language(
        &self,
        git_url: &str,
        language: &str,
        tokens: &[String],
        limit: usize,
        offset: usize,
    ) -> Result<CodeSearchResponse, CodeSearchError>;
}
