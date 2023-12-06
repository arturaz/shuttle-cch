use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Task6Response {
  elf: usize,
  #[serde(rename = "elf on a shelf")]
  elf_on_a_shelf: usize,
  #[serde(rename = "shelf with no elf on it")]
  shelf_with_no_elf_on_it: usize
}

pub async fn task_6(body: String) -> Json<Task6Response> {
  let elf_count = body.match_indices("elf").count();
  let shelf_count = body.match_indices("shelf").count();
  let elf_on_a_shelf_count = body.match_indices("shelf").filter(|(idx, _)| {
    let prefix = "elf on a ";
    let start = idx.saturating_sub(prefix.len());
    let slice = &body[start..*idx];
    slice == prefix
  }).count();

  Json(Task6Response {
    elf: elf_count,
    elf_on_a_shelf: elf_on_a_shelf_count,
    shelf_with_no_elf_on_it: shelf_count - elf_on_a_shelf_count
  })
}