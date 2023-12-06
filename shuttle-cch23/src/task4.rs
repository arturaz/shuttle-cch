use serde::{Deserialize, Serialize};
use axum::Json;

#[derive(Deserialize)]
pub struct Task4Input {
  name: String,
  strength: i64,
}

pub async fn task_4(
  Json(data): Json<Vec<Task4Input>>
) -> String {
  let result: i64 = data.iter().map(|i| i.strength).sum();
  result.to_string()
}

#[derive(Deserialize)]
pub struct Task4_1Input {
  name: String,
  strength: i64,
  speed: f64,
  height: i64,
  antler_width: i64,
  snow_magic_power: i64,
  favorite_food: String,
  #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
  candies_eaten_yesterday: i64,
}

#[derive(Serialize)]
pub struct Task4_1Output {
  fastest: String,
  tallest: String,
  magician: String,
  consumer: String
}

pub async fn task_4_1(
  Json(data): Json<Vec<Task4_1Input>>
) -> Json<Task4_1Output> {
  let fastest =
    data.iter().max_by(|a, b| a.speed.total_cmp(&b.speed)).unwrap();
  let tallest = data.iter().max_by_key(|a| a.height).unwrap();
  let magician = data.iter().max_by_key(|a| a.snow_magic_power).unwrap();
  let consumer = data.iter().max_by_key(|a| a.candies_eaten_yesterday).unwrap();

  let output = Task4_1Output {
    fastest: format!(
      "Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name
    ),
    tallest: format!(
      "{} is standing tall with his {} cm wide antlers", tallest.name, tallest.antler_width
    ),
    magician: format!(
      "{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power
    ),
    consumer: format!(
      "{} ate lots of candies, but also some grass", consumer.name
    )
  };
  Json(output)
}
