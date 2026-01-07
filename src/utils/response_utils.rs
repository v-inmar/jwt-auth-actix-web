use serde::Deserialize;
use serde::Serialize;

use actix_web::HttpRequest;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use actix_web::cookie::CookieBuilder;
use actix_web::cookie::time::Duration;

use crate::constants;




#[derive(Debug, Serialize, Deserialize)]
pub struct RequestDetails {
    pub path: String,
    pub method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusDetails {
    pub code: u16,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDetails<T> {
    pub request_details: RequestDetails,
    pub status_details: StatusDetails,
    pub payload: T,
}

#[derive(Debug, Serialize)]
pub struct PayloadAccessToken{
    pub access_token: String
}

pub struct JsonResponse{}

impl JsonResponse {
    pub fn make_response<T: Serialize>(req: &HttpRequest, code: &StatusCode, payload: T) -> HttpResponse {
        let mut response_builder = HttpResponse::build(*code); // dereference code since it was being passed in as reference
        response_builder.content_type("application/json");
        return response_builder.json(ResponseDetails {
            request_details: JsonResponse::_create_request_details(req),
            status_details: JsonResponse::_create_status_details(code),
            payload: payload,
        });
    }

    // will response with access token as payload and cookie with refresh token
    pub fn make_jwt_response(req: &HttpRequest, code: &StatusCode, access_token: &str, refresh_token: &str) -> HttpResponse{
        let mut response_builder = HttpResponse::build(*code);
        response_builder.content_type("application/json");

        let refresh_cookie = CookieBuilder::new("refresh_token", refresh_token)
        .http_only(true)
        .secure(false) // change true on prod
        .same_site(actix_web::cookie::SameSite::Lax) // change to Strict on prod
        .max_age(Duration::days(7))
        .path("/api/auth")
        .finish();

        response_builder.cookie(refresh_cookie);

        return response_builder.json(ResponseDetails {
            request_details: JsonResponse::_create_request_details(req),
            status_details: JsonResponse::_create_status_details(code),
            payload: PayloadAccessToken{
                access_token: access_token.to_string()
            },
        });



    }

    // we use server error response too often in many handlers, it is best to have a simple function to create the response
    pub fn make_500_response(req: &HttpRequest) -> HttpResponse {
        return JsonResponse::make_response(&req, &StatusCode::INTERNAL_SERVER_ERROR, constants::SERVER_ERROR_MESSAGE.to_string());
    }



    fn _create_request_details(req: &HttpRequest) -> RequestDetails {
        RequestDetails {
            path: req.path().to_string(),
            method: req.method().to_string(),
        }
    }

    fn _create_status_details(code: &StatusCode) -> StatusDetails {
        StatusDetails {
            code: code.as_u16(),
            status: code.canonical_reason().unwrap_or("Status undefined").to_string(),
        }
    }
}





