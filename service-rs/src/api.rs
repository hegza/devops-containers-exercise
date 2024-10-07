use crate::{
    fetch_url::fetch_url,
    sysinfo_response::{self, Response},
    SERVICE_GO_URI,
};
use axum::{
    extract::FromRequest,
    response::{IntoResponse, Json},
};
use hyper::{StatusCode, Uri};
use serde::Serialize;
use sysinfo_response::SysInfo;
use thiserror::Error;

type Result<T> = std::result::Result<T, AppError>;

pub(crate) async fn handler() -> Result<Json<Response>> {
    let (res, body) = fetch_url(Uri::from_static(SERVICE_GO_URI))
        .await
        .map_err(|err| AppError::Fetch(SERVICE_GO_URI.to_owned(), err))?;
    let header = res.headers();
    let content_type = header
        .get("content-type")
        .ok_or(AppError::GetFromServiceGo(
            "invalid header: missing 'content-type'".to_string(),
        ))?;
    if content_type != "application/json" {
        return Err(AppError::GetFromServiceGo(
            "invalid header: 'content-type' needs to be JSON".to_string(),
        ));
    }

    let theirs = serde_json::from_slice(&body).map_err(|e| AppError::Deser(e))?;
    let ours = SysInfo::from_local_info();
    Ok(Json(sysinfo_response::Response::from_infos(ours, theirs)))
}

// The kinds of errors we can hit in our application.
#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("failure querying service-go: {0}")]
    GetFromServiceGo(String),
    #[error("failed to deserialize input")]
    Deser(#[from] serde_json::Error),
    #[error("failed to fetch URI {0}: {1:?}")]
    Fetch(String, Box<dyn std::error::Error + Send + Sync>),
}

// Map `AppError` an axum response
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            AppJson(ErrorResponse {
                message: self.to_string(),
            }),
        )
            .into_response()
    }
}

// Wrap `axum::Json`. This makes it easy to override the rejection and provide our own which formats
// errors to match our application.
//
// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
struct AppJson<T>(T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json(self.0).into_response()
    }
}
