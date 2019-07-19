use r2d2_redis::{r2d2, RedisConnectionManager};

pub use user::*;

pub mod user;

pub type Pool = r2d2::Pool<RedisConnectionManager>;
