use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;

use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Transaction;

use actix_web::HttpRequest;

use bcrypt::verify;

use crate::models::user_authid_model::UserAuthidModel;
use crate::models::user_name_model::UserNameModel;
use crate::models::user_pid_model::UserPidModel;


#[derive(Debug, FromRow, Serialize)]
pub struct UserModel{
    pub id: i32,
    pub firstname_id: i32,
    pub lastname_id: i32,
    pub email_id: i32,
    pub pid_id: i32,
    pub authid_id: i32,
    pub password: String,
    pub datetime_deleted: Option<DateTime<Utc>>,
    pub datetime_deactivated: Option<DateTime<Utc>>,
    pub datetime_verified: Option<DateTime<Utc>>,
    pub datetime_created: DateTime<Utc>,
}


#[derive(Debug, Serialize)]
pub struct ToPrint {
    pub firstname: String,
    pub lastname: String,
    pub url: String,
}

impl UserModel {

    // ----- non-self methods ----- //

    pub async fn new(
        tx: &mut Transaction<'_, Postgres>,firstname_id: i32, 
        lastname_id: i32,
        email_id: i32,
        pid_id: i32,
        authid_id: i32,
        hashed_password: &str //hashed
    ) -> Result<UserModel, sqlx::error::Error>{

        let result = sqlx::query_as!(
            UserModel,  // The target struct type
            r#"
            INSERT INTO user_model (firstname_id, lastname_id, email_id, pid_id, authid_id, password)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, firstname_id, lastname_id, email_id, pid_id, authid_id, password, datetime_deleted, datetime_deactivated, datetime_verified, datetime_created 
            "#,
            firstname_id, lastname_id, email_id, pid_id, authid_id, hashed_password
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(result)
    }



    pub async fn get_by_authid_id(pool: &Pool<Postgres>, authid_id: i32) -> Result<Option<UserModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, firstname_id, lastname_id, email_id, pid_id, authid_id, password, datetime_deleted, datetime_deactivated, datetime_verified, datetime_created
            FROM user_model
            WHERE authid_id = $1
            "#,
            authid_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    pub async fn get_by_pid_id(pool: &Pool<Postgres>, pid_id: i32) -> Result<Option<UserModel>, sqlx::error::Error>{
        let result = sqlx::query_as!(
            UserModel,
            r#"
            SELECT id, firstname_id, lastname_id, email_id, pid_id, authid_id, password, datetime_deleted, datetime_deactivated, datetime_verified, datetime_created
            FROM user_model
            WHERE pid_id = $1
            "#,
            pid_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }


    // ----- self methods ----- /

    pub fn check_password(&self, password: &str) -> Result<bool, bcrypt::BcryptError> {
        return verify(&password, &self.password);
    }

    

    pub async fn get_authid(&self, pool: &Pool<Postgres>) -> Result<UserAuthidModel, Box<dyn std::error::Error>>{
        match UserAuthidModel::get_by_id(&pool, self.authid_id).await?{
            None => Err("User is not associated with authid".into()),
            Some(model) => Ok(model)
        }
    }

    pub async fn to_print(
    &self,
    req: &HttpRequest,
    pool: &Pool<Postgres>,
    ) -> Result<ToPrint, Box<dyn std::error::Error>> {
        let firstname_value = match UserNameModel::get_by_id(&pool, self.firstname_id).await {
            Err(e) => return Err(e.into()),
            Ok(None) => return Err("No firstname associated with user".into()),
            Ok(Some(model)) => model.value,
        };

        let lastname_value = match UserNameModel::get_by_id(&pool, self.lastname_id).await {
            Err(e) => return Err(e.into()),
            Ok(None) => return Err("No lastname associated with user".into()),
            Ok(Some(model)) => model.value,
        };

        let pid_value = match UserPidModel::get_by_id(&pool, self.pid_id).await {
            Err(e) => return Err(e.into()),
            Ok(None) => return Err("No pid associated with user".into()),
            Ok(Some(model)) => model.value,
        };


        // NOTE: URL_FOR doesnt seem to work with scopes
        // let url = match req.url_for("users", &[&pid_value]) {
        //     Ok(u) => u.to_string(),
        //     Err(e) => {
        //         log::error!("{}", e);
        //         return Err(e.into());
        //     }
        // };


        let conn = req.connection_info();
        let url = format!(
            "{}://{}{}",
            conn.scheme(),
            conn.host(),
            format!("/api/users/{}", pid_value)
        );


        let to_print_object = ToPrint{
            firstname: firstname_value,
            lastname: lastname_value,
            url: url
        };



        Ok(to_print_object)
    }



    

    // pub async fn get_email(&self, pool: &Pool<Postgres>) -> Result<UserEmailModel, Box<dyn std::error::Error>>{
    //     match UserEmailModel::get_by_id(&pool, self.email_id).await?{
    //         None => Err("User is not associated with email".into()),
    //         Some(model) => Ok(model)
    //     }
    // }



}