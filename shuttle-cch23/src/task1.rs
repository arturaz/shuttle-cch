use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;
use axum::extract::Path;

pub async fn task_minus_one_error() -> Response {
  (StatusCode::INTERNAL_SERVER_ERROR, "fake error").into_response()
}

pub async fn task_1(
  Path(input): Path<String>
) -> String {
  let parts: Vec<i64> =
    input.split('/').filter_map(|s| s.parse().ok()).collect();

  if parts.is_empty() { "0".to_string() }
  else {
    let tmp = parts.iter().skip(1).fold(parts[0], |a, b| a ^ b);
    let tmp = tmp.pow(3);
    tmp.to_string()
  }
}
