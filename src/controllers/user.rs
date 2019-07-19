use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify};
use r2d2_redis::redis;
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::models::Pool;
use std::ops::{Deref, DerefMut};

/// POST /user/auth
///
#[derive(Deserialize)]
struct AuthRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    id: u32,
    name: String,
    access_token: String,
}

fn auth(
    data: web::Json<AuthRequest>,
    pool: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let mut conn = pool.get().unwrap();

    let res: String = redis::cmd("GET")
        .arg("counter")
        .query(conn.deref_mut()).unwrap();
    println!(">> {:?}", res);

    let password = hash_password(&data.password)?;

    println!("Hashed password: {}", password);

    Ok(HttpResponse::Ok().json(AuthResponse {
        id: 0,
        name: "test".to_owned(),
        access_token: "asd".to_owned(),
    }))
}

/// POST /user/register
///
#[derive(Deserialize)]
struct RegistrationRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum RegistrationResponse {
    Error { error: &'static str },
    Success(AuthResponse),
}

fn register() -> impl Responder {
    HttpResponse::Ok().json(RegistrationResponse::Success({
        AuthResponse {
            id: 0,
            name: "test".to_owned(),
            access_token: "asd".to_owned(),
        }
    }))
}

pub fn scoped_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/user")
            .route("/auth", web::post().to(auth))
            .route("/register", web::post().to(register)),
    );
}

/// Stuff functions
///

fn hash_password(password: &String) -> Result<String, ServiceError> {
    hash(password, 6).map_err(|_| ServiceError::InternalServerError)
}
