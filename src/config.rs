use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

pub async fn get_database_pool() -> MySqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Không có DATABASE_URL trong .env");
    println!("✅ Load database URL: {}", database_url);
    match MySqlPool::connect(&database_url).await {
        Ok(pool) => {
            print!("Kết nối DB thành công");
            return pool;
        }
        Err(e) => {
            eprintln!("❌ Kết nối thất bại: {}", e);
            std::process::exit(1);
        }
    }
}
