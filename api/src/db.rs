use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(database_url: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(database_url)
        .await?;
    tracing::info!("✅ Database connection pool established");
    Ok(pool)
}
