use axum::{Router, routing::get, ServiceExt};
use std::net::SocketAddr;
use dotenv::dotenv;

mod handlers;
mod db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/projects", get(handlers::projects::get_projects));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
