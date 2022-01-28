use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{Article, NewArticle};
use crate::schema::articles::dsl::{articles, uuid as auuid, title, body, published};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);


#[derive(Message)]
#[rtype(result="QueryResult<Article>")]
pub struct Create {
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<Article>")]
pub struct Update {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<Article>")]
pub struct Delete {
    pub uuid: Uuid
}

#[derive(Message)]
#[rtype(result="QueryResult<Article>")]
pub struct Publish {
    pub uuid: Uuid
}

#[derive(Message)]
#[rtype(result="QueryResult<Vec<Article>>")]
pub struct GetArticles;


impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connectio");
        let new_article = NewArticle {
            uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body
        };

        diesel::insert_into(articles)
        .values(new_article)
        .get_result::<Article>(&conn)
    }
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::update(articles)
        .filter(auuid.eq(msg.uuid))
        .set((title.eq(msg.title), body.eq(msg.body)))
        .get_result::<Article>(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");

        diesel::delete(articles)
                .filter(auuid.eq(msg.uuid))
                .get_result::<Article>(&conn)
    }
}

impl Handler<Publish> for DbActor {
    type Result = QueryResult<Article>;

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        diesel::update(articles)
        .filter(auuid.eq(msg.uuid))
        .set(published.eq(true))
        .get_result::<Article>(&conn)
    }
}

impl Handler<GetArticles> for DbActor {
    type Result = QueryResult<Vec<Article>>;

    fn handle(&mut self, msg: GetArticles, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Unable to get a connection");
        articles.filter(published.eq(true))
                .get_results::<Article>(&conn)
    }
}








