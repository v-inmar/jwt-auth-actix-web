use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use sqlx::FromRow;
use sqlx::Postgres;
use sqlx::Transaction;
use sqlx::Pool;

use crate::models::user_model::UserModel;

#[derive(Debug, FromRow, Serialize)]
pub struct UserEmailModel {
    pub id: i32,
    pub value: String,
    pub datetime_created: DateTime<Utc>,
}

impl UserEmailModel {
    pub async fn new(tx: &mut Transaction<'_, Postgres>, value: &str) -> Result<UserEmailModel, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserEmailModel,  // The target struct type
            r#"
            INSERT INTO user_email_model (value)
            VALUES ($1)
            RETURNING id, value, datetime_created
            "#,
            value
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }

    pub async fn get_by_value(pool: &Pool<Postgres>, value: &str) -> Result<Option<UserEmailModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserEmailModel,
            r#"
            SELECT id, value, datetime_created 
            FROM user_email_model
            WHERE value = $1
            "#,
            value
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    // pub async fn get_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<UserEmailModel>, sqlx::error::Error>{
    //     let result = sqlx::query_as!(
    //         UserEmailModel,
    //         r#"
    //         SELECT id, value, datetime_created 
    //         FROM user_email_model
    //         WHERE id = $1
    //         "#,
    //         id
    //     )
    //     .fetch_optional(pool)
    //     .await?;

    //     Ok(result)
    // }

    // Get the user model associated with this email
    // This is function is for convinience
    pub async fn get_user(&self, pool: &Pool<Postgres>) -> Result<Option<UserModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, firstname_id, lastname_id, email_id, pid_id, authid_id, password, datetime_deleted, datetime_deactivated, datetime_verified, datetime_created 
            FROM user_model
            WHERE email_id = $1
            "#,
            self.id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}