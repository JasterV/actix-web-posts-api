extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod models;
mod db_utils;
mod schema;
mod actors;

use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, patch, post, put, web::{self, Data, Json, Path}};

use actix::SyncArbiter;
use actors::db::{Create, DbActor, Delete, GetArticles, Publish, Update};
use db_utils::{get_pool, run_migrations};
use models::{AppState, ArticleData};
use std::env;
use uuid::Uuid;



#[post("/new")]
async fn create_article(article: Json<ArticleData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let article = article.into_inner();

    match db.send(Create { title: article.title, body: article.body }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[post("/{uuid}/publish")]
async fn publish_article(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(Publish { uuid }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[delete("/{uuid}")]
async fn delete_article(Path(uuid): Path<Uuid>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(Delete { uuid }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[put("/{uuid}")]
async fn update_article(Path(uuid): Path<Uuid>, article: Json<ArticleData>, state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();
    let article = article.into_inner();

    match db.send(Update { uuid, title: article.title, body: article.body }).await {
        Ok(Ok(article)) => HttpResponse::Ok().json(article),
        Ok(Err(_)) => HttpResponse::NotFound().json("Article not found"),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[get("/published")]
async fn get_published(state: Data<AppState>) -> impl Responder {
    let db = state.as_ref().db.clone();

    match db.send(GetArticles).await {
        Ok(Ok(articles)) => HttpResponse::Ok().json(articles),
        _ => HttpResponse::InternalServerError().json("Something went wrong")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = env::var("DATABASE_URL").expect("Error retrieving the database url");
    run_migrations(&db_url);
    let pool = get_pool(&db_url);
    let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .service(get_published)
            .service(delete_article)
            .service(publish_article)
            .service(create_article)
            .service(update_article)
            .data(AppState {
                db: db_addr.clone()
            })
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}