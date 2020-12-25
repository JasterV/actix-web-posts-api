extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod actors;
mod db_utils;
mod models;
mod schema;

use actix_web::{
    get,
    web::{self, Path},
    App, HttpServer, Responder,
};

use actix::SyncArbiter;
use actors::db::DbActor;
use db_utils::{get_pool, run_migrations};
use models::AppState;
use std::env;

#[get("/{name}")]
async fn greets(Path(name): Path<String>) -> impl Responder {
    format!("Hello, how are you {}?", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(greets) // Register route
            .data(AppState { db: addr.clone() }) // Set app state
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
