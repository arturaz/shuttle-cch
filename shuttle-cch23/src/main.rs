use axum::{Router, routing::get};
use axum::routing::post;

mod task1;
mod task4;
mod task6;

async fn hello_world() -> &'static str {
  "Hello, world!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(task1::task_minus_one_error))
    .route("/1/*input", get(task1::task_1))
    .route("/4/strength", post(task4::task_4))
    .route("/4/contest", post(task4::task_4_1))
    .route("/6", post(task6::task_6));

  Ok(router.into())
}
