

use actix_web::body::EitherBody;

use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use actix_web::Error;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::middleware::Next;



use crate::utils::response_utils::JsonResponse;



pub async fn method_not_allowed_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<B>>, Error>
where
    B: MessageBody + 'static,
{

    
    // Continue to the next service
    let res = next.call(req).await?;

    let res_status_code = res.status().as_u16();
    if res_status_code == 405{

        
        // Create an HTTP response
        let resp = JsonResponse::make_response(
            &res.request(), 
            &StatusCode::METHOD_NOT_ALLOWED, 
            String::from("Method not allowed for this endpoint")
        );

        // Convert it into a ServiceResponse and return
        return Ok(res.into_response(resp.map_into_right_body()));

    }else{
        Ok(res.map_into_left_body())
    }

}