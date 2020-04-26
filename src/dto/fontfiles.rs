use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct FontDescriptorDTO {
  pub postscript: String,
  pub family: String,
  pub style: String,
  pub weight: usize,
  pub stretch: usize,
  pub italic: bool,
}

#[derive(Serialize)]
pub struct FontFilesDTO {
  pub version: usize,
  #[serde(rename = "fontFiles")]
  pub font_files: HashMap<String, Vec<FontDescriptorDTO>>,
}
