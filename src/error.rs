use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Request failed: {0}")]
    RequestError(reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParseError(reqwest::Error),

    #[error("Database connection error: {0}")]
    DatabaseError(sqlx::Error),

    #[error("Query execution error: {0}")]
    QueryError(sqlx::Error),
}
