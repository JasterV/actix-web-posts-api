extern crate actix;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db_utils;

use actix_web::{
    get,
    web::{self, Path},
    App, HttpResponse, HttpServer, Responder,
};

use db_utils::{get_pool, run_migrations};
use std::env;

#[get("/{name}")]
async fn greets(Path(name): Path<String>) -> impl Responder {
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("PG_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);

    HttpServer::new(move || App::new().service(greets).data(pool.clone()))
        .bind("0.0.0.0:4000")?
        .run()
        .await
}
