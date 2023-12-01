use axum::{routing::get, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

async fn hello_world() -> &'static str {
  "Hello, world!"
}

async fn task_minus_one_error() -> Response {
  (StatusCode::INTERNAL_SERVER_ERROR, "fake error").into_response()
}

async fn task_1(
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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(task_minus_one_error))
    .route("/1/*input", get(task_1));

  Ok(router.into())
}
