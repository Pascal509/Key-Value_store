use sqlx::MySqlPool;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Trying to connect to: {}", database_url);

    match MySqlPool::connect(&database_url).await {
        Ok(_) => println!("✅ Successfully connected to MySQL!"),
        Err(e) => println!("❌ Connection failed: {:?}", e),
    }
}
