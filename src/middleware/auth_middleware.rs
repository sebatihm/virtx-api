use actix_web::dev::ServiceResponse;
use actix_web::error::{ErrorUnauthorized, InternalError};
use actix_web::http::header::{self, AUTHORIZATION};
use actix_web::{Error, HttpMessage, HttpResponse};
use actix_web::{body::MessageBody, dev::ServiceRequest};
use actix_web::middleware::Next;

use crate::utils::jwt::decode_jwt;

//Middleware function to authorize
pub async fn check_auth_middleware( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        let response = HttpResponse::Unauthorized()
            .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
            .json("The User Must specify the JWT");

        return Err(InternalError::from_response("Unauthorized", response).into());
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    match decode_jwt(token){
        Ok(data) => {
            req.extensions_mut().insert(data.claims);
        },
        Err(er) => {
            if er.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature{
                let response = HttpResponse::Unauthorized()
                .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .json("The JTW already Expired");

                return Err(InternalError::from_response("Unauthorized", response).into());

            }else{
                let response = HttpResponse::Unauthorized()
                .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .json("Unauthorized - The JWT is invalid");
            
                return Err(InternalError::from_response("Unauthorized", response).into());
            }
        },
    }
    
    
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}