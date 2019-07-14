#![feature(proc_macro_hygiene, decl_macro)]

use rocket::routes;

mod user;

fn main() {
    rocket::ignite()
        .mount("/api", routes![user::auth, user::create])
        .launch();
}
