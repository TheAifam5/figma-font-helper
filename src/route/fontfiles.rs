use crate::{
  dto::{FontDescriptorDTO, FontFilesDTO},
  provider::FontProviderErr,
  ServerState,
};
use actix_web::{get, web, ResponseError, Result};
use snafu::Snafu;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Snafu)]
pub enum FontFilesHandlerError {
  #[snafu(context(false))]
  FontLoading { source: FontProviderErr },
}

/// font_files handler
#[get("/figma/font-files")]
pub async fn handler(
  state: web::Data<ServerState>,
) -> Result<web::Json<FontFilesDTO>, FontFilesHandlerError> {
  let mut fonts = FontFilesDTO {
    version: state.protocol_version,
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

impl ResponseError for FontFilesHandlerError {}
