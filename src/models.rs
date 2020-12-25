use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::schema::posts;
use crate::actors::db::DbActor;
use crate::actix::Addr;

pub struct AppState {
    pub db: Addr<DbActor>
}

#[derive(Serialize, Deserialize, Debug, Clone, Queryable)]
pub struct Post {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Serialize, Deserialize, Debug, Clone, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostData {
    pub title: String,
    pub body: String,
}

