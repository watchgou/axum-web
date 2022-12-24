use axum::{extract::Path, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub username: String,
}

pub async fn get_user(Path(ids): Path<u32>) -> Json<User> {
    let user = User {
        id: ids,
        username: String::from("hello jon"),
    };
    Json(user)
}
