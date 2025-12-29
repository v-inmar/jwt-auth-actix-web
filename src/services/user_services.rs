
use sqlx::Pool;
use sqlx::Postgres;

use crate::utils::string_utils::StringUtils;
use crate::utils::bcrypt_utils::BcryptUtils;

use crate::dtos::auth_dtos::AuthRegisterDto;
use crate::models::user_model::UserModel;
use crate::models::user_name_model::UserNameModel;
use crate::models::user_email_model::UserEmailModel;
use crate::models::user_authid_model::UserAuthidModel;
use crate::models::user_pid_model::UserPidModel;

pub struct UserServices{}

impl UserServices{
    pub async fn create_new_user_service(data: &AuthRegisterDto, pool: &Pool<Postgres>) -> Result<UserModel, Box<dyn std::error::Error>>{

        // create transaction
        let mut tx = pool.begin().await?;

        // deal with email
        let email = match UserEmailModel::get_by_value(&pool, &data.email).await?{
            Some(m) => m,
            None => UserEmailModel::new(&mut tx, &data.email).await?
        };

        // deal with firstname
        let fname = match UserNameModel::get_by_value(&pool, &data.firstname).await?{
            Some(m) => m,
            None => UserNameModel::new(&mut tx, &data.firstname).await?
        };

        // deal with lastname
        let lname = match UserNameModel::get_by_value(&pool, &data.lastname).await?{
            Some(m) => m,
            None => UserNameModel::new(&mut tx, &data.lastname).await?
        };

        // deal with authid
        let authid: UserAuthidModel;
        let mut authid_counter = 0;
        loop {
            if authid_counter == 5{
                let err_msg = String::from(
                    "Error while creating UserAuthidModel. Try limit has been reached",
                );
                return Err(err_msg.into());
            }

            authid_counter += 1;
            let value = StringUtils::generate_random_string(128);
            match UserAuthidModel::get_by_value(&pool, &value).await?{
                Some(_) => continue,
                None => {
                    authid = UserAuthidModel::new(&mut tx, &value).await?;
                    break;
                }
            };
        }

        // deal with pid
        let pid: UserPidModel;
        let mut pid_counter = 0;
        loop {
            if pid_counter == 5 {
                let err_msg = String::from(
                    "Error while creating UserPidModel. Try limit has been reached",
                );
                return Err(err_msg.into());
            }

            pid_counter += 1;
            let value = StringUtils::generate_random_string(32);
            match UserPidModel::get_by_value(&pool, &value).await?{
                Some(_) => continue,
                None => {
                    pid = UserPidModel::new(&mut tx, &value).await?;
                    break;
                }
            };
        }

        // use bcrypt to hash/encrypt the password
        let hashed_password = BcryptUtils::make_hash(&data.password)?;


        // create the user
        let user = UserModel::new(&mut tx, fname.id, lname.id, email.id, pid.id, authid.id, &hashed_password).await?;

        // commit the transaction
        tx.commit().await?;


        Ok(user)
    }
}

