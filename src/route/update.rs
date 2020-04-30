use crate::{dto::VersionDTO, query::UpdateQuery, ServerState};
use actix_web::{get, web, Result};

/// update handler
#[get("/figma/update")]
pub async fn handler(
  web::Query(query): web::Query<UpdateQuery>,
  state: web::Data<ServerState>,
) -> Result<web::Json<VersionDTO>> {
  if query.version > state.protocol_version {
    // inform about the update somehow
  }

  Ok(web::Json(VersionDTO { version: state.protocol_version }))
}
