use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;

use std::str::FromStr;
use std::time::Duration;

pub struct DatabasePool {
    pub pool: PgPool,
}

impl DatabasePool {
    pub async fn new(url: &str) -> sqlx::Result<DatabasePool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60*5))
            .connect_with(PgConnectOptions::from_str(url)?)
            .await?;

        Ok(DatabasePool{ pool: pool})
    }
    
    pub async fn ping(&self) -> sqlx::Result<String>{
        sqlx::query("SELECT 1")
        .execute(&self.pool)
        .await
        .expect("Unable to establish connection");

        Ok(format!("pong!"))
    }

}