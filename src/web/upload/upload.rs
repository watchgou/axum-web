use axum::{extract::multipart::Multipart, Json};

use crate::web::Result;

pub async fn upload(mut  multipart: Multipart) -> Json<Result<String>> {
    match multipart.next_field().await {
        Ok(file) => {
            if let Some(field) = file {
                let name = field.name().unwrap().to_string();
                //let data = field.bytes().await.unwrap();
                println!("Length of `{}` is bytes", name);
            } else {
                println!("error");
            }
        }
        Err(e) => {
            println!("{}", e);
        }
    }

    Json(Result::default())
}
