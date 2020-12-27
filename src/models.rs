use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::schema::articles;
use crate::actors::db::DbActor;
use crate::actix::Addr;

pub struct AppState {
   pub db: Addr<DbActor>
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]
pub struct Article {
    pub uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="articles"]
pub struct NewArticle {
    pub uuid: Uuid,
    pub title: String,
    pub body: String
}

#[derive(Serialize, Deserialize)]
pub struct ArticleData {
    pub title: String,
    pub body: String
}


