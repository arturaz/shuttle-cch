use axum::extract::Path;
use serde::Deserialize;
use crate::app_error::AppError;

#[derive(Debug, Deserialize)]
struct PokemonApiResponse {
  id: u64,
  name: String,
  #[serde(rename = "weight")]
  weight_in_hectograms: u64
}
impl PokemonApiResponse {
  fn weight(&self) -> f64 {
    self.weight_in_hectograms as f64 / 10.0
  }
}

async fn fetch(id: u64) -> Result<PokemonApiResponse, AppError> {
  let url = format!("https://pokeapi.co/api/v2/pokemon/{id}/", id = id);
  let response = reqwest::get(url).await?;
  let response: PokemonApiResponse = response.json().await?;
  println!("Fetched {:?}", response);
  Ok(response)
}

pub async fn task_8(Path(id): Path<u64>) -> Result<String, AppError> {
  let response = fetch(id).await?;
  Ok(response.weight().to_string())
}

pub async fn task_8_1(Path(id): Path<u64>) -> Result<String, AppError> {
  let response = fetch(id).await?;
  const GRAVITY: f64 = 9.825;
  const METERS: f64 = 10.0;
  let velocity = ((2.0 * GRAVITY * METERS) as f64).sqrt();
  let momentum = response.weight() * velocity;
  Ok(momentum.to_string())
}