use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::constants;


#[derive(Serialize, Deserialize)]
pub struct Claims{
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub id: i32,
    
}

pub fn encode_jwt(email: String, id: i32) -> Result<String, jsonwebtoken::errors::Error>{
    let now = Utc::now();
    let expire_time = Duration::hours(3);

    let user_claims = Claims {
        exp: (now+expire_time).timestamp() as usize,
        iat: now.timestamp() as usize,
        email,
        id,
    };

    let secret = (*constants::SECRET).clone();

    return encode(&Header::default(), &user_claims, &EncodingKey::from_secret(secret.as_ref()));
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>{
    let secret = (*constants::SECRET).clone();
    let claim_data: Result<TokenData<Claims>,jsonwebtoken::errors::Error> = decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()), 
        &Validation::default()
    );

    return claim_data;
}