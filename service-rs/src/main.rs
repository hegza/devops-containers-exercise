mod api;
mod fetch_url;
mod sysinfo_response;

use std::str;

use axum::{routing::get, Router};

const LOCAL_ADDR: &str = "127.0.0.1:8199";
const SERVICE_GO_URI: &str = "http://127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(api::handler));

    // Listen on TCP
    let listener = tokio::net::TcpListener::bind(LOCAL_ADDR).await.unwrap();
    println!("Server listening for TCP at {LOCAL_ADDR}");

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::serve(listener, app).await.unwrap();
}
