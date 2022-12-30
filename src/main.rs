mod web;

#[tokio::main]
async fn main() {
    web::server().await;
    env_logger::init();
}
