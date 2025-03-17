use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn init_db_pool() -> DbPool{
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("there should be a DATABASE URL in environment");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    .expect("Connection to Database failed")

}
