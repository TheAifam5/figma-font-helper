use super::platform::PlatformFontProviderErr;
use std::path::PathBuf;

#[derive(Copy, Clone)]
pub enum FontWeight {
  Thin = 100,
  ExtraLight = 200,
  Light = 300,
  Normal = 400,
  Medium = 500,
  SemiBold = 600,
  Bold = 700,
  ExtraBold = 800,
  Black = 900,
  ExtraBlack = 950,
}

#[derive(Copy, Clone)]
pub enum FontWidth {
  UltraCondensed = 50,
  ExtraCondensed = 63, // 62.5%
  Condensed = 75,
  SemiCondensed = 88, // 87.5%
  Normal = 100,
  SemiExpanded = 113, // 112.5%
  Expanded = 125,
  ExtraExpanded = 150,
  UltraExpanded = 200,
}

pub struct FontDescriptor {
  pub path: PathBuf,
  pub postscript: String,
  pub family: String,
  pub style: String,
  pub weight: FontWeight,
  pub width: FontWidth,
  pub italic: bool,
}

pub trait FontProvider {
  fn new() -> Result<Self, PlatformFontProviderErr>
  where
    Self: Sized;
  fn get_api_version(&self) -> Result<usize, PlatformFontProviderErr>;
  fn get_all_fonts(&self) -> Result<Vec<FontDescriptor>, PlatformFontProviderErr>;
  fn get_font_paths(&self) -> Result<Vec<PathBuf>, PlatformFontProviderErr>;
}
