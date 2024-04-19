use serde::{Deserialize, Serialize};
use crate::error::CustomError;

#[derive(sqlx::FromRow, Debug)]
pub struct Feedback {
   pub text: String,
   pub cosine_similarity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddingResponse {
    pub embedding: Vec<f64>,
}

pub async fn fetch_embedding(url: &str, body: &str) -> Result<EmbeddingResponse, CustomError> {
    let response = reqwest::Client::new().post(url)
        .body(body.to_string())
        .send()
        .await
        .map_err( CustomError::RequestError)?;
   
    let embedding: EmbeddingResponse = response.json().await.map_err(CustomError::JsonParseError)?;  

    Ok(embedding)
}

pub async fn query_embedding(pool: &sqlx::PgPool, query: &str) -> Result<Vec<Feedback>, CustomError> {
    let result = sqlx::query_as::<_, Feedback>(query)
        .fetch_all(pool)
        .await
        .map_err(CustomError::QueryError)?;
    
    Ok(result)
}

pub async fn insert_embedding(pool: &sqlx::PgPool, query: &str) -> Result<bool, CustomError> {
    sqlx::query(query)
        .execute(pool)
        .await
        .map_err(CustomError::QueryError)?;
    
    Ok(true)
}
