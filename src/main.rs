#[macro_use]
extern crate serde;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use r2d2_redis::RedisConnectionManager;

mod controllers;
mod errors;
mod models;
mod utils;

fn main() {
    let _ = dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = RedisConnectionManager::new(database_url.as_str()).unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let app_builder = move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .configure(controllers::scoped_config)
    };

    let listening_address = std::env::var("ADDRESS").unwrap_or(String::from("0.0.0.0"));
    let listening_port = std::env::var("PORT").unwrap_or(String::from("8000"));

    HttpServer::new(app_builder)
        .bind(format!("{}:{}", listening_address, listening_port))
        .unwrap()
        .run()
        .unwrap();
}
