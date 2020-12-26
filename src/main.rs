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
    delete, get, patch, post, put,
    web::{Data, Json, Path},
    App, HttpResponse, HttpServer, Responder,
};

use actix::SyncArbiter;
use actors::db::{Create, DbActor, Delete, GetPosts, Publish, Update};
use db_utils::{get_pool, run_migrations};
use models::{AppState, PostData};
use std::env;
use uuid::Uuid;

#[post("/new")]
async fn create_post(post: Json<PostData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();
    match db
        .send(Create {
            title: post.title,
            body: post.body,
        })
        .await
    {
        Ok(Ok(post)) => HttpResponse::Ok().json(post),
        _ => HttpResponse::InternalServerError().json("Can't create the post"),
    }
}

#[get("/published")]
async fn get_posts(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    match db.send(GetPosts).await {
        Ok(Ok(posts)) => HttpResponse::Ok().json(posts),
        _ => HttpResponse::InternalServerError().json("Can't get the posts"),
    }
}

#[put("/{uuid}")]
async fn update_post(
    Path(uuid): Path<Uuid>,
    post: Json<PostData>,
    state: Data<AppState>,
) -> impl Responder {
    let db = state.as_ref().db.clone();
    let post = post.into_inner();

    db.send(Update {
        uuid,
        title: post.title,
        body: post.body,
    })
    .await
    .map_or(
        HttpResponse::InternalServerError().json("Can't update the post"),
        |result| {
            result.map_or(
                HttpResponse::NotFound().json("Post not found"),
                |post| HttpResponse::Ok().json(post),
            )
        },
    )
}

#[delete("/{uuid}")]
async fn delete_post(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    
    match db.send(Delete { uuid }).await {
        Ok(Ok(post)) => HttpResponse::Ok().json(post),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Can't delete the post"),
    }
}

#[post("/{uuid}/publish")]
async fn publish_post(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    match db.send(Publish { uuid }).await {
        Ok(Ok(post)) => HttpResponse::Ok().json(post),
        Ok(Err(_)) => HttpResponse::NotFound().json("Post not found"),
        _ => HttpResponse::InternalServerError().json("Can't publish the post"),
    }
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
