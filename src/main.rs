use actix_cors::Cors;
use actix_web::HttpServer;
use actix_web::HttpRequest;
use actix_web::App;
use actix_web::Responder;

use actix_web::middleware::from_fn;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::http::StatusCode;
use actix_web::error::InternalError;

use dotenvy::dotenv;

use std::env;

mod handlers;
mod dtos;
mod utils;
mod constants;
mod models;
mod services;
mod middlewares;

// api health check route
async fn health_check(req: HttpRequest) -> impl Responder {
    utils::response_utils::JsonResponse::make_response(&req, &StatusCode::OK, String::from("API is up and running.."))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // load .env file
    dotenv().ok();

    // get server host and port from environment
    let host: String = env::var("SERVER_HOST").unwrap_or("0.0.0.0".to_string());
    let port: u16 = env::var("SERVER_PORT").unwrap_or("5000".to_string()).parse().expect("PORT must be a valid u16 value");

    // get database url connection string from environment
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create database pool
    let db_pool = utils::db_utils::DatabasePool::new(&db_url)
    .await.expect("Unable to establish a connection pool");

    match db_pool.ping().await {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e)
    };

    println!("🚀 Starting database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db_pool.pool)
        .await
        .expect("Failed to run database migrations");
    println!("✅ Database migrations completed successfully!");




    HttpServer::new(move || {
        // for testing - change for more secure options in prod
        let cors = Cors::default()
            .send_wildcard() // for testing only
            // .allowed_origin("*")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
        .wrap(Logger::default())
        .wrap(cors)
        .app_data(web::Data::new(db_pool.pool.clone()))
        
        .service(
            web::scope("/api")

            .wrap(from_fn(middlewares::method_not_allowed_middleware::method_not_allowed_middleware))

            // respond with error if any json request data has missing key/value
            .app_data(
                web::JsonConfig::default().error_handler(|err, req|{
                    let resp = utils::response_utils::JsonResponse::make_response(req, &StatusCode::BAD_REQUEST, &err.to_string().clone());
                    InternalError::from_response(err, resp).into()
                })
            )

            // respond with error if any encoded url form data is missing
            .app_data(
                web::FormConfig::default().error_handler(|err, req| {
                    let resp = utils::response_utils::JsonResponse::make_response(req, &StatusCode::BAD_REQUEST, &err.to_string().clone());
                    InternalError::from_response(err, resp).into()
                })
            )
            .route("/health", web::get().to(health_check))
            .service(
                handlers::auth_handlers::auth_scopes()
                
            )
            .service(
                handlers::user_handlers::user_scopes()
            )
            .default_service(
                web::route().to(|req: HttpRequest| async move {
                    utils::response_utils::JsonResponse::make_response(&req, &StatusCode::NOT_FOUND, String::from("No matching endpoint"))
                })
            )
        )
    }).bind((host, port))?.run().await
}
