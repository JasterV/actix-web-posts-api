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
#[rtype(result = "Option<Post>")]
pub struct Update {
    uuid: Uuid,
    title: String,
    body: String,
}
#[derive(Message)]
#[rtype(result = "Option<Post>")]
pub struct Create {
    title: String,
    body: String,
}

#[derive(Message)]
#[rtype(result = "Option<Post>")]
pub struct Delete {
    uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "Option<Post>")]
pub struct Publish {
    uuid: Uuid,
}

#[derive(Message)]
#[rtype(result = "Option<Vec<Post>>")]
pub struct GetPosts;

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl Handler<Update> for DbActor {
    type Result = Option<Post>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::update(posts)
            .filter(p_uuid.eq(msg.uuid))
            .set((title.eq(msg.title), body.eq(msg.body)))
            .get_result::<Post>(&conn)
            .ok()
    }
}

impl Handler<Create> for DbActor {
    type Result = Option<Post>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        let newPost = NewPost {
            uuid: Uuid::new_v4(),
            title: msg.title,
            body: msg.body,
        };

        diesel::insert_into(posts)
            .values(newPost)
            .get_result::<Post>(&conn)
            .ok()
    }
}

impl Handler<Delete> for DbActor {
    type Result = Option<Post>;

    fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::delete(posts)
                .filter(p_uuid.eq(msg.uuid))
                .get_result::<Post>(&conn)
                .ok()
    }
}

impl Handler<Publish> for DbActor {
    type Result = Option<Post>;

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        diesel::update(posts)
            .filter(p_uuid.eq(msg.uuid))
            .set(published.eq(true))
            .get_result::<Post>(&conn)
            .ok()
    }
}

impl Handler<GetPosts> for DbActor {
    type Result = Option<Vec<Post>>;

    fn handle(&mut self, _msg: GetPosts, _: &mut Self::Context) -> Self::Result {
        let conn = self.0.get().expect("Error retrieving a connection");
        posts.filter(published.eq(true))
        .get_results::<Post>(&conn)
        .ok()
    }
}
