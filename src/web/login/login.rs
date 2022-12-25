use std::time::{SystemTime, UNIX_EPOCH};

use axum::{http::StatusCode, response::IntoResponse, Json};
use jsonwebtoken::{encode, EncodingKey, Header};
use redis::ToRedisArgs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::web::reids_connect;

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
    jti: String,
    exp: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize)]
pub struct Login {
    username: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct User {
    username: String,
    email: String,
}

impl ToRedisArgs for User {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let bytes = bincode::serialize(self).unwrap();
        out.write_arg(&bytes)
    }
}

#[derive(Serialize)]
struct Tokens {
    id: String,
}

pub async fn get_token(Json(_login): Json<Login>) -> impl IntoResponse {
    let mut connect = reids_connect();

    let uid = Uuid::new_v4().to_string();

    let id = format!("access_token:{}", uid.clone());
    let current = SystemTime::now().duration_since(UNIX_EPOCH).expect("OK");

    let my_claims = Claims {
        iss: _login.username.clone(),
        sub: _login.email.clone(),
        company: "ACME".to_owned(),
        jti: uid,
        exp: current.as_secs() + 1200,
    };

    let user = User {
        username: _login.username.clone(),
        email: _login.email.clone(),
    };

    let _: () = redis::cmd("SETEX")
        .arg(id)
        .arg(1200)
        .arg(user)
        .query(&mut connect)
        .unwrap();

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("lonely".as_ref()),
    )
    .unwrap();
    let token_id = Tokens { id: token };
    (StatusCode::OK, Json(token_id))
}
