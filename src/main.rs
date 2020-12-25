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
    post,
    put,
    delete,
    patch,
    web::{self, Path, Json, Data},
    App, HttpServer, Responder, HttpResponse
};

use actix::SyncArbiter;
use actors::db::DbActor;
use db_utils::{get_pool, run_migrations};
use models::{AppState, NewPost, Post};
use std::env;

#[post("/new")]
async fn create_post(post: Json<NewPost>, state: Data<AppState>) -> impl Responder {
    format!("Hello, how are you?")
}

#[get("/published")]
async fn get_posts(state: Data<AppState>) -> impl Responder {
    format!("Hello, how are you")
}

#[put("/{id}")]
async fn update_post(Path(id): Path<String>, post: Json<NewPost>, state: Data<AppState>) -> impl Responder {
    format!("Hello, how are you {}?", id)
}

#[delete("/{id}")]
async fn delete_post(Path(id): Path<String>, state: Data<AppState>) -> impl Responder {
    format!("Hello, how are you {}?", id)
}

#[patch("/{id}")]
async fn publish_post(Path(id): Path<String>, state: Data<AppState>) -> impl Responder {
    format!("Hello, how are you {}?", id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(create_post) // Register route
            .service(update_post) // Register route
            .service(delete_post) // Register route
            .service(publish_post) // Register route
            .service(get_posts) // Register route
            .data(AppState { db: addr.clone() }) // Set app state
    })
    .bind("0.0.0.0:4000")?
    .run()
    .await
}
