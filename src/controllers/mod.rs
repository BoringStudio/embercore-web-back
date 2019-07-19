use actix_web::web;

mod item;
mod user;

pub fn scoped_config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/api")
            .configure(user::scoped_config)
            .configure(item::scoped_config),
    );
}
