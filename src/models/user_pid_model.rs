use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use sqlx::FromRow;
use sqlx::Postgres;
use sqlx::Transaction;
use sqlx::Pool;

#[derive(Debug, FromRow, Serialize)]
pub struct UserPidModel {
    pub id: i32,
    pub value: String,
    pub datetime_created: DateTime<Utc>,
}

impl UserPidModel {
    pub async fn new(tx: &mut Transaction<'_, Postgres>, value: &str) -> Result<UserPidModel, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserPidModel,  // The target struct type
            r#"
            INSERT INTO user_pid_model (value)
            VALUES ($1)
            RETURNING id, value, datetime_created
            "#,
            value
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }

    pub async fn get_by_value(pool: &Pool<Postgres>, value: &str) -> Result<Option<UserPidModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserPidModel,
            r#"
            SELECT id, value, datetime_created 
            FROM user_pid_model
            WHERE value = $1
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }


    pub async fn get_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserPidModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserPidModel,
            r#"
            SELECT id, value, datetime_created 
            FROM user_pid_model
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}