use anyhow::{Ok, Result};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use warp::Filter;

//TODO use .env not this variable
const DATABASE_URL: &str = "postgresql://admin:admin@localhost:5432/Inventory";

pub async fn connect_db() -> Result<DatabaseConnection> {
    let db: DatabaseConnection = Database::connect(DATABASE_URL).await?;
    Migrator::up(&db, None).await?;
    Ok(db)
}

pub fn with_db(
    db: DatabaseConnection,
) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
