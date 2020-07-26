use crate::provider::{FontDescriptor, FontProvider, PlatformFontProviderErr};
use std::{ops::Deref, path::PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FontDatabaseErr {
  #[error("Failed to initialize font database: {0}")]
  Initialization(String),

  #[error(transparent)]
  PlatformFontProvider(#[from] PlatformFontProviderErr),
}

type Result<T, E = FontDatabaseErr> = std::result::Result<T, E>;

pub struct FontDatabase {
  provider: Box<dyn FontProvider>,
  fonts: Vec<FontDescriptor>,
}

impl FontDatabase {
  pub fn new(provider: Box<dyn FontProvider>) -> Result<Self> {
    let mut instance = Self { provider, fonts: vec![] };
    instance.invalidate()?;
    Ok(instance)
  }

  pub fn invalidate(&mut self) -> Result<()> {
    self.fonts = self.provider.get_all_fonts()?;
    Ok(())
  }

  pub fn is_path_valid(&self, path: PathBuf) -> bool {
    self.fonts.iter().any(|f| f.path == path)
  }
}

impl Deref for FontDatabase {
  type Target = Vec<FontDescriptor>;

  fn deref(&self) -> &Self::Target {
    &self.fonts
  }
}
