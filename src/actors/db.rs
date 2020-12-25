use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{NewPost, Post};
use crate::schema::posts::dsl::{body, posts, published, title, uuid as p_uuid};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use uuid::Uuid;
pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Update {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}
#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Create {
    pub title: String,
    pub body: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Delete {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct Publish {
    pub uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct GetPosts;

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Update> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::update(posts)
            .filter(p_uuid.eq(msg.uuid))
            .set((title.eq(msg.title), body.eq(msg.body)))
            .get_result::<Post>(&conn)
    }
}

impl Handler<Create> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        let new_post = NewPost {
            uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(posts)
            .values(new_post)
            .get_result::<Post>(&conn)
    }
}

impl Handler<Delete> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::delete(posts)
                .filter(p_uuid.eq(msg.uuid))
                .get_result::<Post>(&conn)
    }
}

impl Handler<Publish> for DbActor {
    type Result = QueryResult<Post>;

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::update(posts)
            .filter(p_uuid.eq(msg.uuid))
            .set(published.eq(true))
            .get_result::<Post>(&conn)
    }
}

impl Handler<GetPosts> for DbActor {
    type Result = QueryResult<Vec<Post>>;

    fn handle(&mut self, _msg: GetPosts, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        posts.filter(published.eq(true))
        .get_results::<Post>(&conn)
    }
}
