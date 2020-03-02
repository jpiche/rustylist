extern crate chrono;
#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::{env, io};

mod db;
mod models;
mod routes;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let bind = env::var("SERVER_BIND").expect("SERVER_BIND environment variable must be set");
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable must be set");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db connection pool.");

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(routes::index)

            .service(routes::users::all)
            .service(routes::users::create)
            .service(routes::users::get)
            .service(routes::users::delete)

            .service(routes::tasks::all)
            .service(routes::tasks::create)
            .service(routes::tasks::get)
            .service(routes::tasks::delete)
            .service(routes::tasks::update)
            .service(routes::tasks::get_for_user)
    })
    .bind(&bind)?
    .run()
    .await
}
