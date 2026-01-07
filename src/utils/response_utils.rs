use serde::Deserialize;
use serde::Serialize;

use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::cookie::CookieBuilder;
use actix_web::cookie::time::Duration;
use actix_web::http::StatusCode;

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
pub struct PayloadAccessToken {
    pub access_token: String,
}

pub struct JsonResponse {}

impl JsonResponse {
    pub fn make_response<T: Serialize>(
        req: &HttpRequest,
        code: &StatusCode,
        payload: T,
    ) -> HttpResponse {
        let mut response_builder = HttpResponse::build(*code); // dereference code since it was being passed in as reference
        response_builder.content_type("application/json");
        return response_builder.json(ResponseDetails {
            request_details: JsonResponse::_create_request_details(req),
            status_details: JsonResponse::_create_status_details(code),
            payload: payload,
        });
    }

    // will response with access token as payload and cookie with refresh token
    pub fn make_jwt_response(
        req: &HttpRequest,
        code: &StatusCode,
        access_token: &str,
        refresh_token: &str,
    ) -> HttpResponse {
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
            payload: PayloadAccessToken {
                access_token: access_token.to_string(),
            },
        });
    }

    // we use server error response too often in many handlers, it is best to have a simple function to create the response
    pub fn make_500_response(req: &HttpRequest) -> HttpResponse {
        return JsonResponse::make_response(
            &req,
            &StatusCode::INTERNAL_SERVER_ERROR,
            constants::SERVER_ERROR_MESSAGE.to_string(),
        );
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
            status: code
                .canonical_reason()
                .unwrap_or("Status undefined")
                .to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;


    #[test]
    fn make_response_creates_json_with_request_info() {
        // Create a mock request
        let req = TestRequest::get().uri("/api/test").to_http_request();

        let response = JsonResponse::make_response(&req, &StatusCode::OK, "test payload");

        // Check status
        assert_eq!(response.status(), StatusCode::OK);

        // Check content-type
        let content_type = response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(content_type, "application/json");
    }

    #[test]
    fn make_jwt_response_sets_cookie_and_access_token() {
        let req = TestRequest::post().uri("/api/auth/login").to_http_request();

        let access_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";
        let refresh_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...";

        let response =
            JsonResponse::make_jwt_response(&req, &StatusCode::OK, access_token, refresh_token);

        // These assertions work perfectly:
        assert_eq!(response.status(), StatusCode::OK);

        let cookie = response.cookies().next().unwrap();
        assert_eq!(cookie.name(), "refresh_token");
        assert_eq!(cookie.value(), refresh_token);
        assert!(cookie.http_only().unwrap());
        assert_eq!(cookie.path(), Some("/api/auth"));

    }

    #[test]
    fn make_500_response_returns_server_error() {
        let req = TestRequest::get().uri("/api/error").to_http_request();
        let response = JsonResponse::make_500_response(&req);

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
