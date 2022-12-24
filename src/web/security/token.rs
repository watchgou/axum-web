use std::marker::PhantomData;

use crate::web::connect;
use axum::{
    http::{header, Request, Response, StatusCode},
    response::Result,
};
use http_body::Body;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_http::auth::AuthorizeRequest;
pub struct MyBearer<ResBody> {
    pub _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Clone for MyBearer<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    company: String,
    jti: String,
    exp: u64,
}

impl<B, ResBody> AuthorizeRequest<B> for MyBearer<ResBody>
where
    ResBody: Body + Default,
{
    type ResponseBody = ResBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let header = request.headers().get(header::AUTHORIZATION);
        // DECODE

        if let Some(v1) = header {
            // println!("{:#?}", v1);

            match v1.to_str() {
                Ok(v) => {
                    let tk = &v[7..];

                    let token_message = decode::<Claims>(
                        tk,
                        &DecodingKey::from_secret("abcdefghijklmnoprstxyz".as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    match token_message {
                        Ok(token_data) => {
                            let claims = token_data.claims;
                            let uuid = claims.jti;

                            let mut connect = connect();
                            let uuid = format!("access_token:{}", uuid);

                            let result: i64 =
                                redis::cmd("TTL").arg(uuid).query(&mut connect).unwrap();
                            if result < 0 {
                                let mut res = Response::new(ResBody::default());
                                *res.status_mut() = StatusCode::NON_AUTHORITATIVE_INFORMATION;
                                Err(res)
                            } else {
                                Ok(())
                            }
                        }
                        Err(_) => {
                            let mut res = Response::new(ResBody::default());
                            *res.status_mut() = StatusCode::NETWORK_AUTHENTICATION_REQUIRED;
                            Err(res)
                        }
                    }
                }
                Err(_) => {
                    let mut res = Response::new(ResBody::default());
                    *res.status_mut() = StatusCode::NON_AUTHORITATIVE_INFORMATION;
                    Err(res)
                }
            }
        } else {
            let mut res = Response::new(ResBody::default());
            *res.status_mut() = StatusCode::UNAUTHORIZED;
            Err(res)
        }
    }
}
