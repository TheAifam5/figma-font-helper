use crate::{
  dto::{FontDescriptorDTO, FontFilesDTO},
  query::FontFilesQuery,
  ServerState,
};
use actix_web::{error, get, web, Result};
use std::{collections::HashMap, path::PathBuf};

/// font_files handler
#[get("/figma/font-files")]
pub async fn handler(
  web::Query(query): web::Query<FontFilesQuery>,
  state: web::Data<ServerState>,
) -> Result<web::Json<FontFilesDTO>> {
  if state.font_provider_api_version < query.ft_min_ver {
    return Err(error::ErrorBadRequest("Unsupported FreeType version"));
  }

  let mut fonts = FontFilesDTO {
    version: state.figma_api_version,
    font_files: HashMap::<PathBuf, Vec<FontDescriptorDTO>>::new(),
  };

  for descriptor in state.database.iter() {
    let font_desc = FontDescriptorDTO {
      postscript: descriptor.postscript.clone(),
      family: descriptor.family.clone(),
      style: descriptor.style.clone(),
      weight: descriptor.weight as usize,
      stretch: descriptor.width as usize,
      italic: descriptor.italic,
    };

    if let Some(font_vec) = fonts.font_files.get_mut(&descriptor.path) {
      font_vec.push(font_desc);
    } else {
      fonts.font_files.insert(descriptor.path.clone(), vec![font_desc]);
    }
  }
  Ok(web::Json(fonts))
}
