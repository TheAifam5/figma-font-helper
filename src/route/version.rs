use crate::{dto::VersionDTO, ServerState};
use actix_web::{get, web, Result};

/// version handler
#[get("/figma/version")]
pub async fn handler(state: web::Data<ServerState>) -> Result<web::Json<VersionDTO>> {
  Ok(web::Json(VersionDTO { version: state.protocol_version }))
}
