use crate::{dto::VersionDTO, ServerState};
use actix_web::{get, web, Result};

/// version handler
#[get("/figma/version")]
pub async fn handler(config: web::Data<ServerState>) -> Result<web::Json<VersionDTO>> {
  Ok(web::Json(VersionDTO { version: config.protocol_version }))
}
