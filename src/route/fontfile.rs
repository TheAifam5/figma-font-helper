use crate::{query::FontFileQuery, ServerState};
use actix_files as fs;
use actix_web::{error, get, web, Result};

/// font_file handler
#[get("/figma/font-file")]
pub async fn handler(
  web::Query(query): web::Query<FontFileQuery>,
  state: web::Data<ServerState>,
) -> Result<fs::NamedFile> {
  if state.font_provider_api_version < query.ft_min_ver {
    return Err(error::ErrorBadRequest("Unsupported FreeType version"));
  }

  if let Some(desc) = state.database.iter().find(|f| f.path == query.file) {
    Ok(fs::NamedFile::open(&desc.path)?)
  } else {
    Err(error::ErrorNotFound("File not found"))
  }
}
