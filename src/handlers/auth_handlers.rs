use actix_web::Responder;
use actix_web::HttpRequest;
use actix_web::web;
use actix_web::Scope;
use actix_web::http::StatusCode;
use actix_web::middleware::from_fn;




use sqlx::PgPool;
use validator::Validate;
use validator::ValidationError;

use crate::dtos::auth_dtos::AuthRegisterDto;
use crate::dtos::auth_dtos::AuthLoginDto;
use crate::utils::response_utils::JsonResponse;
use crate::models::user_email_model::UserEmailModel;
use crate::models::user_authid_model::UserAuthidModel;
use crate::models::user_revoked_token_model::UserRevokedTokenModel;
use crate::models::user_model::UserModel;

use crate::services::user_services::UserServices;
use crate::services::revoked_services::RevokedServices;

use crate::utils::jwt_utils::JwtUtils;

use crate::middlewares::auth_middleware::auth_middleware;



// use of .service to enable 405
pub fn auth_scopes() -> Scope {
    web::scope("/auth")
        .service(
            web::resource("/register")
            .route(web::post().to(AuthHandlers::register_handler))
        )

        .service(
            web::resource("/login")
            .route(web::post().to(AuthHandlers::login_handler))
        )
        
        .service(
            web::resource("/refresh_token")
            .route(web::post().to(AuthHandlers::refresh_token_handler))
        )
        .service(
            web::resource("/logout")
                .route(web::post().to(AuthHandlers::logout_handler))  // route FIRST
                .wrap(from_fn(auth_middleware))                      // THEN wrap. wrap only works on service (resource)
        )
}

pub struct AuthHandlers{}

impl AuthHandlers {

    /// Handler for registering a new user
    pub async fn register_handler(req: HttpRequest, data: web::Json<AuthRegisterDto>, pool: web::Data<PgPool>) -> impl Responder {
        
        // validate
        if let Some(s) = data.validate().err(){
            return JsonResponse::make_response(&req, &StatusCode::BAD_REQUEST, s.errors());
        }

        // check password
        if data.password != data.repeat {
            return JsonResponse::make_response(&req, &StatusCode::BAD_REQUEST, ValidationError::new("Password and Repeat must match"));
        }

        // check email <-> user
        match UserEmailModel::get_by_value(&pool, &data.email).await {
            Err(_) => {
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(None) => {}
            Ok(Some(user_email_model)) => {

                match user_email_model.get_user(&pool).await{
                    Err(_) => {
                        return JsonResponse::make_500_ressponse(&req);
                    }
                    Ok(None) => {}
                    Ok(Some(_)) => {
                        return JsonResponse::make_response(&req, &StatusCode::CONFLICT, String::from("Email address already in use"))
                    }
                }
            }
        }

        // create the user
        match UserServices::create_new_user_service(&data, &pool).await{
            Err(_) => {
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(user_model) => {
                // ceate tokens here
                match UserAuthidModel::get_by_id(&pool, user_model.authid_id).await{
                    Err(_) => {
                        return JsonResponse::make_500_ressponse(&req);
                    }
                    Ok(None) => {

                        // no authid model associated with user (VERY BAD!!!)
                        return JsonResponse::make_500_ressponse(&req);
                    }Ok(Some(user_authid_model)) => {


                        let access_token = match JwtUtils::gen_access_token(&user_authid_model.value){
                            Err(_) => {
                                return JsonResponse::make_500_ressponse(&req);
                            }
                            Ok(at) => at

                        };

                        let refresh_token = match JwtUtils::gen_refresh_token(&user_authid_model.value){
                            Err(_) => {
                                return JsonResponse::make_500_ressponse(&req);
                            }
                            Ok(rt) => rt

                        };

                        return JsonResponse::make_jwt_response(&req, &StatusCode::CREATED, &access_token, &refresh_token);
                    }
                }

                
            }
        }
    }

    /// Handler for logging in an existing user
    pub async fn login_handler(req: HttpRequest, data: web::Form<AuthLoginDto>, pool: web::Data<PgPool>) -> impl Responder {

        // check if email exist
        let user_email_model = match UserEmailModel::get_by_value(&pool, &data.email).await{
            Err(_) => {
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(None) => {
                return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Invalid email and/or password"));
            }
            Ok(Some(model)) => model

        };

        // check if user associated with the email exist
        let user_model = match user_email_model.get_user(&pool).await{
            Err(_) => {
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(None) => {
                return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Invalid email and/or password"));
            }
            Ok(Some(model)) => model
        };

        // check if the password matched with the user's hashed password
        match user_model.check_password(&data.password){
            Err(_) => {
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(result) => {
                if !result{
                    return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Invalid email and/or password"));
                }else{
                    // create tokens
                    match user_model.get_authid(&pool).await{
                        Err(_) => {
                            return JsonResponse::make_500_ressponse(&req);
                        }
                        Ok(user_authid_model) => {
                            // create token
                            let access_token = match JwtUtils::gen_access_token(&user_authid_model.value){
                                Err(_) => {
                                    return JsonResponse::make_500_ressponse(&req);
                                }
                                Ok(at) => at

                            };

                            let refresh_token = match JwtUtils::gen_refresh_token(&user_authid_model.value){
                                Err(_) => {
                                    return JsonResponse::make_500_ressponse(&req);
                                }
                                Ok(rt) => rt

                            };

                            return JsonResponse::make_jwt_response(&req, &StatusCode::OK, &access_token, &refresh_token);
                        }
                    }
                    
                }
            }
        }

    }

    /// Handler for refreshing access and refresh jwt tokens of the user
    pub async fn refresh_token_handler(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {

        // get the cookie
        if let Some(refresh_token_cookie) = req.cookie("refresh_token") {

            // check if refresh token is not revoked yet
            match UserRevokedTokenModel::get_by_value(&pool, refresh_token_cookie.value()).await {
                Err(_) => {
                    return JsonResponse::make_500_ressponse(&req);
                }
                Ok(Some(_)) => {
                    // already revoked, cannot use this token
                    return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Refresh token already revoked"));
                }
                Ok(None) => {
                    // check the token
                    match JwtUtils::decode_refresh_token(&refresh_token_cookie.value()){
                        Err(e) => {
                            if e.to_string().eq_ignore_ascii_case("invalidsignature")
                            || e.to_string().eq_ignore_ascii_case("expiredsignature")
                            || e.to_string().eq_ignore_ascii_case("invalidtoken")
                            || e.to_string().starts_with("Base64")
                            {
                                return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, format!("Refresh token: {}", e.to_string()));
                            }else{
                                return JsonResponse::make_500_ressponse(&req);
                            }
                        }
                        Ok(token_data) => {
                            // get authid and check that it exist and belongs to a user
                            match UserAuthidModel::get_by_value(&pool, &token_data.claims.sub).await{
                                Err(e) => {
                                    log::error!("{}", e);
                                    return JsonResponse::make_500_ressponse(&req);
                                }Ok(None) => {
                                    return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Authid value is not recognised"));
                                }
                                Ok(Some(model)) => {
                                    match UserModel::get_by_authid_id(&pool, model.id).await{
                                        Err(e) => {
                                            log::error!("{}", e);
                                            return JsonResponse::make_500_ressponse(&req);
                                        }
                                        Ok(None) => {
                                            return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("No associated user with authid value"));
                                        }
                                        Ok(Some(user_model)) => {
                                            if matches!(user_model.datetime_deleted, Some(_)) || matches!(user_model.datetime_deactivated, Some(_)){
                                                return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("User is not active"));
                                            }
                                        }
                                    }
                                }
                            }


                            // get the authid and create new access and refresh tokens
                            let access_token = match JwtUtils::gen_access_token(&token_data.claims.sub){
                                Err(_) => {
                                    return JsonResponse::make_500_ressponse(&req);
                                }
                                Ok(at) => at

                            };

                            let refresh_token = match JwtUtils::gen_refresh_token(&token_data.claims.sub){
                                Err(_) => {
                                    return JsonResponse::make_500_ressponse(&req);
                                }
                                Ok(rt) => rt

                            };

                            // revoke the old refresh token
                            match RevokedServices::create_new_revoke_token_service(&refresh_token_cookie.value(), &pool).await {
                                Err(e) => {
                                    log::error!("{}", e);
                                    return JsonResponse::make_500_ressponse(&req);
                                }
                                Ok(_) => {
                                    return JsonResponse::make_jwt_response(&req, &StatusCode::OK, &access_token, &refresh_token);
                                }
                            }

                        }
                    }
                }
            }
        }else{
            return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Missing cookie"));
        }
    }

    /// Handler for logging out a user
    pub async fn logout_handler(req: HttpRequest, pool: web::Data<PgPool>) -> impl Responder {

        if let Some(refresh_token_cookie) = req.cookie("refresh_token") {

            // check if token has been revoked
            match UserRevokedTokenModel::get_by_value(&pool, &refresh_token_cookie.value()).await{
                Err(_) => {
                    return JsonResponse::make_500_ressponse(&req);
                }
                Ok(None) => {} // Not revoked yet
                Ok(Some(_)) => {
                    // refresh token already revoked
                    return JsonResponse::make_response(&req, &StatusCode::OK, String::from("Logged out successfully"));
                }
            }



            match JwtUtils::decode_refresh_token(refresh_token_cookie.value()) {
                Err(e) => {
                    if e.to_string().eq_ignore_ascii_case("expiredsignature"){
                        // expired anyways, respond with OK
                        return JsonResponse::make_response(&req, &StatusCode::OK, String::from("Logged out successfully"));
                    }else{
                        return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, format!("Refresh token: {}", e.to_string()));
                    }
                }
                Ok(_) => {
                    // revoke the refresh token
                    match RevokedServices::create_new_revoke_token_service(&refresh_token_cookie.value(), &pool).await{
                        Err(_) => {
                            return JsonResponse::make_500_ressponse(&req);
                        }
                        Ok(_) => {
                            return JsonResponse::make_response(&req, &StatusCode::OK, "Logged out successfully");
                        }
                    }
                }
            }

            
            



        }else{
            return JsonResponse::make_response(&req, &StatusCode::UNAUTHORIZED, String::from("Missing cookie"));
        }


        
    }
}