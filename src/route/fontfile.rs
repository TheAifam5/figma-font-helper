use crate::{query::FontFileQuery, ServerState};
use actix_files as fs;
use actix_web::{get, web, Error, HttpResponse, Result};

/// font_file handler
#[get("/figma/font-file")]
pub async fn handler(
  web::Query(query): web::Query<FontFileQuery>,
  state: web::Data<ServerState>,
) -> Result<fs::NamedFile> {
  if state.database.iter().any(|f| f.path.parent() == query.file.parent()) {
    Ok(fs::NamedFile::open(query.file)?)
  } else {
    Err(Error::from(HttpResponse::Forbidden()))
  }
}
