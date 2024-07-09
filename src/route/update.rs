use crate::{dto::VersionDTO, query::UpdateQuery, ServerState};
use actix_web::{get, web, Result};

/// update handler
#[get("/figma/update")]
pub async fn handler(
  web::Query(query): web::Query<UpdateQuery>,
  state: web::Data<ServerState>,
) -> Result<web::Json<VersionDTO>> {
  if query.version > state.figma_api_version {
    // inform about the update somehow
  }

  Ok(web::Json(VersionDTO { version: state.figma_api_version }))
}
