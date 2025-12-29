use sqlx::Postgres;
use sqlx::Pool;

use chrono::Utc;
use chrono::Duration;

use crate::models::user_revoked_token_model::UserRevokedTokenModel;


pub struct RevokedServices {}

impl RevokedServices {
    pub async fn create_new_revoke_token_service(token: &str, pool: &Pool<Postgres>) -> Result<UserRevokedTokenModel, Box<dyn std::error::Error>>{
        let mut tx = pool.begin().await?;

        let utc_now = Utc::now();
        let ttl = utc_now + Duration::days(14); // this gurantees token had already expired before it is removed from db

        let revoked = UserRevokedTokenModel::new(&mut tx, &token, &ttl).await?;

        tx.commit().await?;

        Ok(revoked)
    }
}