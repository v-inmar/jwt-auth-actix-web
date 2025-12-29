use actix_web::HttpMessage; // for extension_mut()

use actix_web::body::EitherBody;

use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;
use actix_web::http::header;


use crate::utils::response_utils::JsonResponse;
use crate::utils::jwt_utils::JwtUtils;


pub async fn auth_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error>
where
    B: MessageBody + 'static,
{
    // Pre-processing

    // get authorization header
    let auth_header_value = match req.headers().get(header::AUTHORIZATION) {
        None => {
            // Create an HTTP response
            let resp = JsonResponse::make_response(
                &req.request(), 
                &StatusCode::UNAUTHORIZED, 
                String::from("Missing authorization header")
            );

            // Convert it into a ServiceResponse and return early
            return Ok(req.into_response(resp.map_into_right_body()));
        }
        Some(token) => match token.to_str(){
            Err(_) => {
                // Create an HTTP response
                let resp = JsonResponse::make_response(
                    &req.request(), 
                    &StatusCode::UNAUTHORIZED, 
                    String::from("Invalid authorization header")
                );

                // Convert it into a ServiceResponse and return early
                return Ok(req.into_response(resp.map_into_right_body()));
            }
            Ok(value) => value
        }
    };

    // get access token
    let access_token = match auth_header_value.strip_prefix("Bearer "){
        None => {
            // Create an HTTP response
            let resp = JsonResponse::make_response(
                &req.request(), 
                &StatusCode::UNAUTHORIZED, 
                String::from("Invalid authorization header value. Missing 'Bearer' string")
            );

            // Convert it into a ServiceResponse and return early
            return Ok(req.into_response(resp.map_into_right_body()));
        }
        Some(value) => value
    };

    // validate access token
    let token_data = match JwtUtils::decode_access_token(&access_token) {
        Ok(td) => td,

        Err(e) => {
            let is_refresh_request =
                e.to_string().eq_ignore_ascii_case("expiredsignature")
                && req.path().eq_ignore_ascii_case("/api/auth/refresh_token");

            if is_refresh_request {
                // Skip auth for refresh-token endpoint
                let res = next.call(req).await?;
                return Ok(res.map_into_left_body());
            }

            // Otherwise: unauthorized
            let resp = JsonResponse::make_response(
                &req.request(),
                &StatusCode::UNAUTHORIZED,
                format!("Access token: {}", e.to_string()),
            );

            return Ok(req.into_response(resp.map_into_right_body()));
        }
    };


    req.extensions_mut().insert(token_data.claims.sub.to_string());

    // Continue to the next service
    let res = next.call(req).await?;

    // Post-processing
    Ok(res.map_into_left_body())
}





