use std::path::PathBuf;

use axum::{routing::get, Router};
use contents::File;
use vector::VectorDB;

mod contents;
mod errors;
mod open_ai;
mod vector;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn embed_documentation(vector_db: &mut VectorDB, files: &Vec<File>) -> anyhow::Result<()> {
    for file in files {
        let embeddings = open_ai::embed_file(file).await?;
        println!("Embedding: {:?}", file.path);
        for embedding in embeddings.data {
            vector_db.upsert_embedding(embedding, file).await?;
        }
    }

    Ok(())
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_static_folder::StaticFolder(folder = "docs")] docs_folder: PathBuf,
    #[shuttle_static_folder::StaticFolder(folder = ".")] prefix: PathBuf,
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    let files = contents::load_files_from_dir(docs_folder, &prefix, ".mdx")?;
    let mut vector_db = VectorDB::new(&secrets)?;
    open_ai::setup(&secrets)?;

    println!("Setup done");

    vector_db.reset_collection().await?;
    embed_documentation(&mut vector_db, &files).await?;

    println!("Embeddings done");

    Ok(router.into())
}
