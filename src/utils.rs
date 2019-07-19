use jsonwebtoken::{decode, encode, Header, Validation};
use serde::Serialize;

use crate::errors::ServiceError;
use crate::models::User;

/// JWT claims encoding/decoding
///

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,

    // subject
    sub: String,

    // issued at
    iat: i64,

    // expiry,
    exp: i64,

    // user id
    user_id: u32,
}

impl Claims {
    fn with_user_id(user_id: u32) -> Self {
        use chrono::prelude::*;
        use time::Duration;

        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::days(1)).timestamp(),
            user_id,
        }
    }
}

pub fn create_token(user: &User) -> Result<String, ServiceError> {
    let claims = Claims::with_user_id(user.id);

    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<u32, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.user_id))
        .map_err(|_| ServiceError::Unauthorized)?
}

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or("no secret".into())
}
