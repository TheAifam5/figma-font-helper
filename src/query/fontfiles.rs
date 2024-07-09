use serde::Deserialize;

#[derive(Deserialize)]
pub struct FontFilesQuery {
  #[serde(rename = "freetype_minimum_api_version")]
  pub ft_min_ver: usize,
  pub isolate: bool,
}
