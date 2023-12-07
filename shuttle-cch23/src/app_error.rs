use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub struct AppError(anyhow::Error);

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError where E: Into<anyhow::Error> {
  fn from(err: E) -> Self {
    Self(err.into())
  }
}

impl IntoResponse for AppError {
  fn into_response(self) -> Response {
    (StatusCode::BAD_REQUEST, self.0.to_string()).into_response()
  }
}