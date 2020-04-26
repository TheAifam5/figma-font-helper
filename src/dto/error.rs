use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorDTO {
  pub version: usize,
  pub error: String,
}
