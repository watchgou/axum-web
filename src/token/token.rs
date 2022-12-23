use std::marker::PhantomData;

use axum::http::{header, Request, Response};
use http_body::Body;
use tower_http::auth::AuthorizeRequest;

pub struct MyBearer<ResBody> {
    pub _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Clone for MyBearer<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

impl<B, ResBody> AuthorizeRequest<B> for MyBearer<ResBody>
where
    ResBody: Body + Default,
{
    type ResponseBody = ResBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let header = request.headers().get(header::AUTHORIZATION);
        if let Some(sm) = header {
            println!("{:#?}",sm)
        } else {
            println!("--")
        }
        Ok(())
    }
}
