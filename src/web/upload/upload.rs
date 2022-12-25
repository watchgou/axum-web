use axum::{extract::multipart::Multipart, Json};

use crate::web::Result;

pub async fn upload(mut multipart: Multipart) -> Json<Result<String>> {
    match multipart.next_field().await {
        Ok(file) => {
            if let Some(field) = file {
                let name = field.name().unwrap().to_string();
                //let data = field.bytes().await.unwrap();
                let data = field.bytes();
                match data.await {
                    Ok(bytes) => {
                        println!("Length of `{}` is {} bytes", name, bytes.len());
                        Json(Result::default())
                    }
                    Err(_) => {
                        Json(Result {
                            code: 1,
                            msg: "Failed read".to_owned(),
                            data: None,
                        })
                    }
                }
            } else {
                Json(Result {
                    code: 1,
                    msg: "Failed read".to_owned(),
                    data: None,
                })
            }
        }
        Err(_) => Json(Result {
            code: 1,
            msg: "file is empty".to_owned(),
            data: None,
        }),
    }
}
