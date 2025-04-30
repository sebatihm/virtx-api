use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{Error, HttpMessage};
use actix_web::{body::MessageBody, dev::ServiceRequest};
use actix_web::middleware::Next;

use crate::utils::jwt::decode_jwt;

//Middleware function to authorize
pub async fn check_auth_middleware( req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error>{
    let auth = req.headers().get(AUTHORIZATION);
    
    if auth.is_none(){
        return Err(ErrorUnauthorized("Unauthorized - The user must specify the JWT"));
    }

    let token = auth.unwrap().to_str().unwrap().replace("Bearer ", "").to_owned();
    match decode_jwt(token){
        Ok(data) => {
            req.extensions_mut().insert(data.claims);
        },
        Err(er) => {
            if er.kind() == &jsonwebtoken::errors::ErrorKind::ExpiredSignature{
                return Err(ErrorUnauthorized("Unauthorized - The JWT has expired"));
            }else{
                return Err(ErrorUnauthorized("Unauthorized - The JWT is invalid"));
            }
        },
    }
    
    
    
    next.call(req).await
    .map_err(|err |ErrorUnauthorized(err) )
}