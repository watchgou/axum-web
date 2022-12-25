

mod web;

#[tokio::main]
async fn main() {

    web::server().await;
}
