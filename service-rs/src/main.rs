mod sysinfo_response;

use axum::{response::Json, routing::get, Router};
use sysinfo_response::SysInfo;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    // Listen on TCP
    let addr = "127.0.0.1:8199";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server listening for TCP at {addr}");

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Json<sysinfo_response::Response> {
    let theirs = SysInfo::from_mock();
    let ours = SysInfo::from_local_info();
    Json(sysinfo_response::Response::from_infos(ours, theirs))
}
