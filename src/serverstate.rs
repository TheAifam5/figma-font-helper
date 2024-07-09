use crate::provider::{
  FontDatabase, FontDatabaseErr, FontProvider, PlatformFontProvider, PlatformFontProviderErr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerStateErr {
  #[error(transparent)]
  ProviderError(#[from] PlatformFontProviderErr),

  #[error(transparent)]
  DatabaseError(#[from] FontDatabaseErr),
}

type Result<T, E = ServerStateErr> = std::result::Result<T, E>;

pub struct ServerState {
  pub figma_api_version: usize,
  pub font_provider_api_version: usize,
  pub database: FontDatabase,
}

impl ServerState {
  pub fn new() -> Result<Self, ServerStateErr> {
    let font_provider = Box::new(PlatformFontProvider::new()?);
    Ok(Self {
      figma_api_version: 4,
      font_provider_api_version: font_provider.get_api_version()?,
      database: FontDatabase::new(font_provider)?,
    })
  }
}
