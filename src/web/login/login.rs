use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::web::connect;



/*
iss (issuer)：签发人

exp (expiration time)：过期时间

sub (subject)：主题

aud (audience)：受众

nbf (Not Before)：生效时间

iat (Issued At)：签发时间

jti (JWT ID)：编号

*/

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    company: String,
    jti:String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Login {
    username: String,
    email: String,
}

#[derive(Serialize)]
struct Tokens {
    id: String,
}

pub async fn get_token(Json(_login): Json<Login>) ->  impl IntoResponse {

    let _connect=connect();
   
    let uid=Uuid::new_v4().to_string();

    let my_claims = Claims {
        iss: "jon".to_string(),
        sub: "b@b.com".to_owned(),
        company: "ACME".to_owned(),
        jti:uid,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    let token_id = Tokens { id: token };
    (StatusCode::OK, Json(token_id))
}
