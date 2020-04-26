use serde::Deserialize;

#[derive(Deserialize)]
pub struct FontFileQuery {
  pub file: String,
}
