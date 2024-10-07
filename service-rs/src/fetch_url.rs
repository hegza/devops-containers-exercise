use http_body_util::{BodyExt, Empty};
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::{io::AsyncWriteExt as _, net::TcpStream};

/// Generic `Result` with a message
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Makes a GET request to a URI and returns the response and its streamed body. Logs on stdout.
pub(crate) async fn fetch_url(
    uri: hyper::Uri,
) -> Result<(hyper::Response<hyper::body::Incoming>, Vec<u8>)> {
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
