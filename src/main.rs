use axum::{routing::get, Json, Router};
use serde_json::json;
use std::net::SocketAddr;
use tokio;

// Handler for the /user endpoint
async fn get_user() -> Json<serde_json::Value> {
    Json(json!({"id": 1, "name": "Alice"}))
}

#[tokio::main]
async fn main() {
    // Define the routes
    let app = Router::new().route("/user", get(get_user));

    // Define the address to listen on
    //let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    //println!("Server running at http://{}", addr);

    // Start the server
    //axum::serve(tcp_listener, make_service)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
