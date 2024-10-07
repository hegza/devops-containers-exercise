mod api;
mod sysinfo_response;

use std::str;

use axum::{routing::get, Router};
use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::{io::AsyncWriteExt as _, net::TcpStream};

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

// A simple type alias so as to DRY
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Makes GET request to a URI
///
/// Returns the response and its streamed body
async fn fetch_url(uri: hyper::Uri) -> Result<(hyper::Response<hyper::body::Incoming>, Vec<u8>)> {
    let host = uri.host().expect("uri has no host");
    let port = uri.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);

    println!("GET {}...", addr);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection failed: {:?}", err);
        }
    });

    let authority = uri.authority().unwrap().clone();

    let path = uri.path();
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<axum::body::Bytes>::new())?;

    let mut res = sender.send_request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to a buffer
    let mut body_buf: Vec<u8> = Vec::new();
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            body_buf.write(chunk).await.unwrap();
        }
    }

    Ok((res, body_buf))
}
