use crate::{query::FontFileQuery, ServerState};
use actix_files as fs;
use actix_web::{get, web, Error, HttpResponse, Result};

/// font_file handler
#[get("/figma/font-file")]
pub async fn handler(
  web::Query(query): web::Query<FontFileQuery>,
  state: web::Data<ServerState>,
) -> Result<fs::NamedFile> {
  if let Some(desc) = state.database.iter().find(|f| f.path == query.file) {
    Ok(fs::NamedFile::open(&desc.path)?)
  } else {
    Err(Error::from(HttpResponse::NotFound()))
  }
}
