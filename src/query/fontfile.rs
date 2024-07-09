use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct FontFileQuery {
  #[serde(rename = "freetype_minimum_api_version")]
  pub ft_min_ver: usize,
  pub file: PathBuf,
}
