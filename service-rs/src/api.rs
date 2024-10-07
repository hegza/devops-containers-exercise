use crate::{
    fetch_url,
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

///
type Result<T> = std::result::Result<T, AppError>;

pub(crate) async fn handler() -> Result<Json<Response>> {
    let (res, body) = fetch_url(Uri::from_static(SERVICE_GO_URI))
        .await
        .map_err(AppError::Generic)?;
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
    /*
    // Some error from a third party library we're using
    TimeError(time_library::Error),
    */
    #[error("failure querying service-go")]
    GetFromServiceGo(String),
    #[error("failed to deserialize input")]
    Deser(#[from] serde_json::Error),
    #[error("internal server error")]
    Generic(Box<dyn std::error::Error + Send + Sync>),
}

// Tell axum how `AppError` should be converted into a response.
//
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // How we want errors responses to be serialized
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            AppError::GetFromServiceGo(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
            AppError::Deser(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::Generic(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
        };

        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

// Wrap `axum::Json`. This makes it easy to override the rejection and provide
// our own which formats errors to match our application.
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
