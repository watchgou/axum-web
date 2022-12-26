use crate::web;
use axum::{extract::multipart::Multipart, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    userid: String,
}

pub async fn upload(mut multipart: Multipart) -> Json<web::Result<Vec<User>>> {
    let result = web::datasource::query_info(
        "select username,password,userid from user",
        |(username, password, userid)| User {
            username,
            password,
            userid,
        },
    );

    match multipart.next_field().await {
        Ok(file) => {
            if let Some(field) = file {
                let name = field.name().unwrap().to_string();
                //let data = field.bytes().await.unwrap();
                let data = field.bytes();
                match data.await {
                    Ok(bytes) => {
                        println!("Length of `{}` is {} bytes", name, bytes.len());

                        let data = match result {
                            Ok(values) => values,
                            Err(_) => panic!(),
                        };

                        //Json(web::Result::default())
                        Json(web::Result {
                            code: 0,
                            msg: "success".to_owned(),
                            data: Some(data),
                        })
                    }
                    Err(_) => Json(web::Result {
                        code: 1,
                        msg: "Failed read".to_owned(),
                        data: None,
                    }),
                }
            } else {
                Json(web::Result {
                    code: 1,
                    msg: "Failed read".to_owned(),
                    data: None,
                })
            }
        }
        Err(_) => Json(web::Result {
            code: 1,
            msg: "file is empty".to_owned(),
            data: None,
        }),
    }
}
