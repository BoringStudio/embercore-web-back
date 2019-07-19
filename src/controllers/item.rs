use actix_web::{web, HttpResponse, Responder};

fn get_all() -> impl Responder {
    HttpResponse::Ok().json("{}")
}

fn get_one(path: web::Path<(u32)>) -> impl Responder {
    println!("Id request: {}", path);

    HttpResponse::Ok().json("{}")
}

fn create() -> impl Responder {
    HttpResponse::Ok().json("{}")
}

pub fn scoped_config(conf: &mut web::ServiceConfig) {
    conf.route("/items", web::get().to(get_all))
        .route("/item/{id}", web::get().to(get_one))
        .route("/item", web::post().to(create));
}
