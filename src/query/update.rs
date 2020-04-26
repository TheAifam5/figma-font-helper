use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateQuery {
  pub version: usize,
}
