use crate::query::FontFileQuery;
use actix_files as fs;
use actix_web::{get, web, Result};

/// font_file handler
#[get("/figma/font-file")]
pub async fn handler(web::Query(query): web::Query<FontFileQuery>) -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open(query.file.as_str())?)
}
