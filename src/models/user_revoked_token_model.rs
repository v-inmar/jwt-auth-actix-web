use chrono::DateTime;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Transaction;
use sqlx::FromRow;
use chrono::Utc;

use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
pub struct UserRevokedTokenModel {
    pub id: i32,
    pub value: String,
    pub datetime_ttl: DateTime<Utc>,
    pub datetime_created: DateTime<Utc>,
}

impl UserRevokedTokenModel {
    pub async fn new(tx: &mut Transaction<'_, Postgres>, value: &str, datetime_ttl: &DateTime<Utc>) -> Result<UserRevokedTokenModel, sqlx::error::Error> {
        let result = sqlx::query_as!(
            UserRevokedTokenModel,  // The target struct type
            r#"
            INSERT INTO user_revoked_token_model (value, datetime_ttl)
            VALUES ($1, $2)
            RETURNING id, value, datetime_ttl, datetime_created
            "#,
            value,
            datetime_ttl
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)

    }

    pub async fn get_by_value(pool: &Pool<Postgres>, value: &str) -> Result<Option<UserRevokedTokenModel>, sqlx::error::Error> {
        let result = sqlx::query_as!(
            UserRevokedTokenModel,
            r#"
            SELECT id, value, datetime_ttl, datetime_created
            FROM user_revoked_token_model
            WHERE value = $1
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}