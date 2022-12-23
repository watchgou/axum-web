use axum::{
    routing::{get, post},
    Router,
};
use std::{marker::PhantomData, net::SocketAddr};
use tower_http::auth::RequireAuthorizationLayer;

mod jsons;

mod paths;

mod login;

mod token;




#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let login = Router::new().route("/login", post(login::get_token));

    let app = Router::new()
        .route("/", get(jsons::root))
        .route("/users", post(jsons::create_user))
        //  请求参数带有内容
        .route("/getPath/:id", get(paths::get_user))
        .route_layer(RequireAuthorizationLayer::custom(token::MyBearer {
            _ty: PhantomData,
        }));

    let all = Router::new().merge(login).merge(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(all.into_make_service())
        .await
        .unwrap();
}
