use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct FontFileQuery {
  pub file: PathBuf,
}
