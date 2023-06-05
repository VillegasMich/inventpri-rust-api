use sea_orm::{DatabaseConnection};
use warp::{self, Filter, filters::BoxedFilter};

pub fn router(db: DatabaseConnection) -> BoxedFilter<(impl warp::Reply,)>  {
    get_all_items(db.clone()).boxed().or(post_item(db.clone())).boxed()
}

fn json_body<T: serde::de::DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 1000).and(warp::body::json::<T>())
}

fn with_db(
    db: DatabaseConnection,
) -> impl Filter<Extract = (DatabaseConnection,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn get_all_items(db: DatabaseConnection) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path("item")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(crate::handlers::get_all_items)
        .boxed()
}

pub fn post_item(db: DatabaseConnection) -> BoxedFilter<(impl warp::Reply,)> {
    warp::path("item")
        .and(warp::path::end())
        .and(warp::post())
        .and(json_body::<crate::models::Post>())
        .and(with_db(db.clone()))
        .and_then(crate::handlers::post_item)
        .boxed()
}