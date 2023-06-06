use anyhow::{Ok, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use warp::reply::with_status;

pub async fn get_all_items(db: DatabaseConnection) -> Result<impl warp::Reply, anyhow::Error> {
    let items: Vec<crate::entity::item::Model> =
        super::entity::item::Entity::find().all(&db).await?;
    Ok(warp::reply::json(&items))
}

pub async fn get_item_by_id(
    id: u32,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, anyhow::Error> {
    let item = super::entity::item::Entity::find_by_id(id as i32)
        .one(&db)
        .await?;
    Ok(with_status(
        warp::reply::json(&item),
        warp::http::StatusCode::OK,
    ))
}

pub async fn post_item(
    json_item: crate::models::Item,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, anyhow::Error> {
    let new_item: crate::entity::item::ActiveModel = crate::entity::item::ActiveModel {
        name: Set(json_item.name.to_lowercase().to_owned()),
        price: Set(json_item.price.to_string().to_owned()), //TODO when migration done change this to u32
        location: Set(json_item.location.to_lowercase().to_owned()),
        ..Default::default()
    };

    let new_item: crate::entity::item::Model = new_item.insert(&db).await?;

    Ok(with_status(
        warp::reply::json(&new_item),
        warp::http::StatusCode::CREATED,
    ))
}

pub async fn delete_item_by_id(
    id: u32,
    db: DatabaseConnection,
) -> Result<impl warp::Reply, anyhow::Error> {
    let _item = super::entity::item::Entity::delete_by_id(id as i32)
        .exec(&db)
        .await?;
    Ok(with_status("Item deleted!", warp::http::StatusCode::OK))
}
