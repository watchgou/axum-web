use std::marker::PhantomData;

use axum::{
    http::{header, Request, Response, StatusCode},
    response::Result,
};
use http_body::Body;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower_http::auth::AuthorizeRequest;

use crate::web::connect;
pub struct Authorization<ResBody> {
    pub _ty: PhantomData<fn() -> ResBody>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    sub: String,
    company: String,
    jti: String,
    exp: u64,
}

impl<ResBody> Clone for Authorization<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

#[derive(Serialize)]
struct Msg {
    msg: String,
}

impl<B, ResBody> AuthorizeRequest<B> for Authorization<ResBody>
where
    ResBody: Body + Default,
{
    type ResponseBody = ResBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let header = request.headers().get(header::AUTHORIZATION);
        // DECODE

        if let Some(value) = header {
            match value.to_str() {
                Ok(v1) => {
                    let token_id = &v1[7..];

                    let token_message = decode::<Claims>(
                        token_id,
                        &DecodingKey::from_secret("lonely".as_ref()),
                        &Validation::new(Algorithm::HS256),
                    );

                    match token_message {
                        Ok(token_data) => {
                            let mut connect = connect();

                            let result: i64 = redis::cmd("TTL")
                                .arg(format!("access_token:{}", token_data.claims.jti))
                                .query(&mut connect)
                                .unwrap();
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
