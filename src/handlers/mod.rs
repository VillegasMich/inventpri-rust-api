use warp::{self};

pub async fn get_all_items(db: sea_orm::DatabaseConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let items = crate::utilities::get_all_items(db).await;
    match items {
        Ok(items) => {
            Ok(items)
        },
        Err(_e) => {
            return Err(warp::reject())
        }
    }    
}

pub async fn post_item(json_item: crate::models::Post, db: sea_orm::DatabaseConnection) -> Result<impl warp::Reply, warp::Rejection> {
    let item = crate::utilities::post_item(json_item, db).await;
    match item {
        Ok(item) => {
            Ok(item)
        },
        Err(_e) => {
            return Err(warp::reject())
        }
    }
}