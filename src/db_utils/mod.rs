use diesel::{
    connection::Connection,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub fn run_migrations(db_url: &str) {
    embed_migrations!();
    let connection = PgConnection::establish(db_url).expect("Error connecting to database");
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Error running migrations");
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a connection pool")
}
