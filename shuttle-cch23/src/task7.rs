use anyhow::anyhow;
use axum::http::HeaderMap;
use axum::{Json};
use base64::Engine;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::app_error::AppError;

fn get_cookie(headers: &HeaderMap) -> anyhow::Result<String> {
  let cookie_header =
    headers.get("Cookie").ok_or(anyhow!("Can't find Cookie header"))?.to_str()?;
  let base64_str =
    cookie_header.split_once('=').ok_or(anyhow!("Can't find = in the cookie body"))?.1;
  let base64_engine = base64::engine::general_purpose::STANDARD;
  let contents = String::from_utf8(base64_engine.decode(base64_str)?)?;
  Ok(contents)
}

pub async fn task_7(
  headers: HeaderMap
) -> Result<String, AppError> {
  let response = get_cookie(&headers)?;
  Ok(response)
}

type IngredientsMap = std::collections::HashMap<String, u64>;

#[derive(Deserialize)]
struct Task7_1Input {
  recipe: IngredientsMap,
  pantry: IngredientsMap,
}
impl Task7_1Input {
  fn cookies(&self) -> u64 {
    let cookies =
      self.recipe.iter().fold(u64::MAX, |cookies, (ingredient, needed)| {
        let available = self.pantry.get(ingredient).unwrap_or(&0);
        cookies.min(available / needed)
      });

    cookies
  }

  fn left_over_after(&self, cookies: u64) -> IngredientsMap {
    let mut left_over = self.pantry.clone();

    self.recipe.iter().for_each(|(ingredient, needed)| {
      match left_over.get(ingredient) {
        None => {}
        Some(available) => {
          left_over.insert(ingredient.clone(), available - cookies * needed);
        }
      }
    });

    left_over
  }
}

pub async fn task_7_1(
  headers: HeaderMap
) -> Result<Json<Value>, AppError> {
  let json = get_cookie(&headers)?;
  let input: Task7_1Input = serde_json::from_str(&json)?;

  let cookies = input.cookies();
  let left_over = input.left_over_after(cookies);

  let response = json!({
    "cookies": cookies,
    "pantry": left_over
  });

  Ok(Json(response))
}