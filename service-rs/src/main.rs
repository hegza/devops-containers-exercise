mod api;
mod fetch_url;
mod sysinfo_response;

use std::{str, sync};

use axum::{routing::get, Router};

const LISTEN_ADDR: &str = "0.0.0.0:3000";
const SERVICE_GO_URI: sync::LazyLock<&str> =
    sync::LazyLock::new(|| option_env!("GO_URI").unwrap_or("http://service-go:3000"));

#[tokio::main]
async fn main() {
    let service_lock = sync::Arc::new(sync::Mutex::new(()));

    let app = Router::new().route(
        "/",
        get({
            let lock = sync::Arc::clone(&service_lock);
            move || crate::api::handler(lock)
        }),
    );

    // Listen on TCP
    let listener = tokio::net::TcpListener::bind(LISTEN_ADDR).await.unwrap();
    println!("Server listening for TCP at {LISTEN_ADDR}");

    // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::serve(listener, app).await.unwrap();
}
