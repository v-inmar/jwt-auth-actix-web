use actix_web::web;
use actix_web::HttpRequest;
use actix_web::Responder;
use actix_web::Scope;
use actix_web::http::StatusCode;
use actix_web::middleware::from_fn;

use sqlx::PgPool;

use crate::models::user_pid_model::UserPidModel;
use crate::models::user_model::UserModel;

use crate::middlewares::auth_middleware::auth_middleware;

use crate::utils::response_utils::JsonResponse;

pub fn user_scopes() -> Scope {
    web::scope("/users")
        .service(
            web::resource("/{pid}")
            .route(web::get().to(UserHandlers::get_user))
            .wrap(from_fn(auth_middleware))
        )
}

pub struct UserHandlers{}

impl UserHandlers {
    pub async fn get_user(req: HttpRequest, pid: web::Path<String>, pool: web::Data<PgPool>) -> impl Responder {
        let pid_value = pid.into_inner(); // extract the actual path value. Path<String> is just a wrapper


        // Get the pid model
        let user_pid_model = match UserPidModel::get_by_value(&pool, &pid_value).await{
            Err(e) => {
                log::error!("{}", e);
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(None) => {
                return JsonResponse::make_response(&req, &StatusCode::NOT_FOUND, String::from("User not found"));
            }
            Ok(Some(model)) => model
        };


        let user_model = match UserModel::get_by_pid_id(&pool, user_pid_model.id).await{
            Err(e) => {
                log::error!("{}", e);
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(None) => {
                return JsonResponse::make_response(&req, &StatusCode::NOT_FOUND, String::from("User not found"));
            }
            Ok(Some(model)) => model
        };

        let to_print = match user_model.to_print(&req, &pool).await{
            Err(e) => {
                log::error!("{}", e);
                return JsonResponse::make_500_ressponse(&req);
            }
            Ok(obj) => obj
        };

        JsonResponse::make_response(&req, &StatusCode::OK, to_print)


    }
}