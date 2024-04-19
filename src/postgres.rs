use sqlx::postgres::PgPoolOptions;
use crate::error::CustomError;

pub async fn connect_db(url: &str) -> Result<sqlx::Pool<sqlx::Postgres> , CustomError> {
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(url)
        .await
        .map_err(CustomError::DatabaseError)?;
    
    Ok(pool)
}
