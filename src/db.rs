use sqlx::{MySql, Pool};
use std::env;
use dotenv::dotenv;

pub async fn get_db_pool() -> Pool<MySql> {
    // Load environment variables
    dotenv().ok();

    // Print to check if variable is being read correctly
    match env::var("DATABASE_URL") {
        Ok(val) => println!("DATABASE_URL: {}", val),
        Err(e) => panic!("DATABASE_URL not found: {:?}", e),
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("Connecting to database: {}", database_url);

    sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database")
}