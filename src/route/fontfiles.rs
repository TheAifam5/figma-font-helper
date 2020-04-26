use crate::dto::{FontDescriptorDTO, FontFilesDTO};
use crate::ServerConfig;
use actix_web::web::post;
use actix_web::{get, web, ResponseError, Result};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use font_kit::{
  error::{FontLoadingError, SelectionError},
  handle::Handle,
  properties::Style,
  source::SystemSource,
};
use log::{error, info};
use snafu::Snafu;
use std::collections::HashMap;

#[derive(Debug, Snafu)]
pub enum FontFilesHandlerError {
  #[snafu(context(false))]
  FontLoading { source: FontLoadingError },

  #[snafu(context(false))]
  FontSelection { source: SelectionError },
}

/// font_files handler
#[get("/figma/font-files")]
pub async fn handler(
  config: web::Data<ServerConfig>,
  font_system: web::Data<SystemSource>,
) -> Result<web::Json<FontFilesDTO>, FontFilesHandlerError> {
  let mut fonts = FontFilesDTO {
    version: config.protocol_version,
    font_files: HashMap::<String, Vec<FontDescriptorDTO>>::new(),
  };

  for handle in font_system.all_fonts()? {
    let font_path;

    match handle {
      Handle::Path { ref path, .. } => font_path = path.to_str().unwrap(),
      _ => continue,
    };

    info!("Loading font {}", font_path);

    let font = handle.load()?;
    let props;

    match std::panic::catch_unwind(|| font.properties()) {
      Ok(p) => props = p,
      Err(e) => {
        error!("Unable to get properties for the font {:#?}", e);
        continue;
      }
    }

    let postscript: String;

    if let Some(value) = font.postscript_name() {
      postscript = value;
    } else if let Some(data) = font.copy_font_data() {
      let mut hasher = Sha256::new();
      hasher.input(data.as_ref());
      postscript = hasher.result_str();
    } else {
      postscript = format!("{}_{}", font.family_name(), convert_style(props.style));
    }

    let font_desc = FontDescriptorDTO {
      postscript,
      family: font.family_name(),
      style: convert_style(props.style),
      weight: props.weight.0.round() as usize,
      stretch: (props.stretch.0 * 1.0).round() as usize,
      italic: props.style != Style::Normal,
    };

    if let Some(font_vec) = fonts.font_files.get_mut(font_path) {
      font_vec.push(font_desc);
    } else {
      fonts.font_files.insert(font_path.to_owned(), vec![font_desc]);
    }
  }
  Ok(web::Json(fonts))
}

impl ResponseError for FontFilesHandlerError {}

#[inline]
fn convert_style(style: Style) -> String {
  if style == Style::Normal {
    return "Regular".to_owned();
  }

  style.to_string()
}
