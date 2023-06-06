pub mod database;
pub mod entity;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utilities;

#[tokio::main]
async fn main() {
    const PORT: u16 = 3030;
    let db: Result<sea_orm::DatabaseConnection, anyhow::Error> = database::connect_db().await;
    match db {
        Ok(db_connection) => {
            println!("Connection complete to database ✅");
            println!("The server is running on port {} 🚀", PORT);
            warp::serve(routes::router(db_connection))
                .run(([127, 0, 0, 1], PORT))
                .await;
        }
        Err(err) => println!("Error connection to database ❌ \n {}", err.to_string()),
    }
}
