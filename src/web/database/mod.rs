pub mod test_mysql {
    use axum::Json;

    use serde::{Deserialize, Serialize};

    use crate::web;

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize,Default)]
    pub struct User {
        username: String,
        password: String,
        userid: String,
        colid:String,
    }

    pub async fn query_user() -> Json<web::Result<Vec<User>>> {
        let result = web::datasource::query_info(
            "select username,password,userid from user limit 0,10",
            |(username, password, userid)| User {
                username,
                password,
                userid,
                colid:String::default(),
            },
        );
        let data = match result {
            Ok(values) => values,
            Err(_) => panic!(),
        };

        Json(web::Result {
            code: 0,
            msg: "success".to_owned(),
            data: Some(data),
        })
    }
}
