use anyhow::{Ok, Result};
use openai::embeddings::Embedding;
use qdrant_client::{
    prelude::{QdrantClient, QdrantClientConfig},
    qdrant::{vectors_config::Config, CreateCollection, Distance, VectorParams, VectorsConfig},
};
use shuttle_secrets::SecretStore;

use crate::{contents::File, errors::SetupError};

static COLLECTION: &str = "docs";

pub struct VectorDB {
    client: QdrantClient,
    id: u64,
}

impl VectorDB {
    pub fn new(secrets: &SecretStore) -> Result<Self> {
        let qdrant_token = secrets
            .get("QDRANT_TOKEN")
            .ok_or(SetupError("QDRANT_TOKEN not available"))?;
        let qdrant_url = secrets
            .get("QDRANT_URL")
            .ok_or(SetupError("QDRANT_URL not available"))?;

        let mut qdrant_config = QdrantClientConfig::from_url(&qdrant_url);
        qdrant_config.set_api_key(&qdrant_token);

        let client = QdrantClient::new(Some(qdrant_config))?;

        Ok(Self { client, id: 0 })
    }

    pub async fn reset_collection(&self) -> Result<()> {
        self.client.delete_collection(COLLECTION).await?;

        self.client
            .create_collection(&CreateCollection {
                collection_name: COLLECTION.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: 1536,
                        distance: Distance::Cosine.into(),
                        hnsw_config: None,
                        quantization_config: None,
                        on_disk: None,
                    })),
                }),
                ..Default::default()
            })
            .await?;

        Ok(())
    }

    pub async fn upsert_embedding(&mut self, embedding: Embedding, file: &File) -> Result<()> {
        Ok(())
    }
}
