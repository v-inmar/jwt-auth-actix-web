use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Transaction;

#[derive(Debug, FromRow, Serialize)]
pub struct UserNameModel{
    pub id: i32,
    pub value: String,
    pub datetime_created: DateTime<Utc>,
}

impl UserNameModel {
    pub async fn new(tx: &mut Transaction<'_, Postgres>, value: &str) -> Result<UserNameModel, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserNameModel,  // The target struct type
            r#"
            INSERT INTO user_name_model (value)
            VALUES ($1)
            RETURNING id, value, datetime_created
            "#,
            value
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }

    pub async fn get_by_value(pool: &Pool<Postgres>, value: &str) -> Result<Option<UserNameModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserNameModel,
            r#"
            SELECT id, value, datetime_created 
            FROM user_name_model
            WHERE value = $1
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    
    pub async fn get_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserNameModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserNameModel,
            r#"
            SELECT id, value, datetime_created 
            FROM user_name_model
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

}

