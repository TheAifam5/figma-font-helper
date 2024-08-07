#![allow(unsafe_code)]

use crate::provider::{FontDescriptor, FontProvider, FontWeight, FontWidth};

use std::{
  convert::TryFrom,
  ffi::{CStr, OsStr},
  os::raw::{c_char, c_int, c_uchar},
  path::{Path, PathBuf},
  ptr,
  slice::from_raw_parts,
  str::Utf8Error,
  string::ToString,
};
use strum_macros::Display;
use thiserror::Error;

#[repr(u32)]
#[derive(PartialEq)]
enum FcResult {
  Match,
}

#[repr(i32)]
#[derive(Display)]
enum FcWeight {
  Thin = 0,
  ExtraLight = 40,
  Light = 50,
  Book = 75,
  Regular = 80,
  Medium = 100,
  DemiBold = 180,
  Bold = 200,
  ExtraBold = 205,
  Black = 210,
  ExtraBlack = 215,
}

#[repr(i32)]
enum FcWidth {
  UltraCondensed = 50,
  ExtraCondensed = 63, // 62.5%
  Condensed = 75,
  SemiCondensed = 87, // 87.5% - wonder which ** cant round the numbers up from font-config team :facepalm:
  Normal = 100,
  SemiExpanded = 113, // 112.5%
  Expanded = 125,
  ExtraExpanded = 150,
  UltraExpanded = 200,
}

#[repr(C)]
struct FcConfig {
  _private: [u8; 0],
}

#[repr(C)]
struct FcStrList {
  _private: [u8; 0],
}

#[repr(C)]
struct FcPattern {
  _private: [u8; 0],
}

#[repr(C)]
struct FcObjectSet {
  _private: [u8; 0],
}

#[repr(C)]
struct FcFontSet {
  nfont: c_int,
  sfont: c_int,
  fonts: *const *const FcPattern,
}

type FcChar8 = c_uchar;
type FcBool = c_int;
const FC_FAMILY: &[u8] = b"family\0";
const FC_STYLE: &[u8] = b"style\0";
const FC_FILE: &[u8] = b"file\0";
const FC_WEIGHT: &[u8] = b"weight\0";
const FC_WIDTH: &[u8] = b"width\0";
const FC_SLANT: &[u8] = b"slant\0";
const FC_POSTSCRIPT_NAME: &[u8] = b"postscriptname\0";
const FC_SLANT_ITALIC: c_int = 100;

#[link(name = "fontconfig")]
extern "C" {
  fn FcInitLoadConfigAndFonts() -> *mut FcConfig;
  fn FcConfigDestroy(config: *const FcConfig);

  fn FcConfigEnableHome(enable: FcBool) -> FcBool;
  fn FcConfigGetFontDirs(config: *const FcConfig) -> *const FcStrList;

  fn FcPatternCreate() -> *mut FcPattern;
  fn FcPatternDestroy(p: *const FcPattern);

  fn FcObjectSetBuild(first: *const FcChar8, ...) -> *mut FcObjectSet;
  fn FcObjectSetDestroy(os: *const FcObjectSet);

  fn FcFontList(
    config: *const FcConfig,
    p: *const FcPattern,
    os: *const FcObjectSet,
  ) -> *mut FcFontSet;
  fn FcFontSetDestroy(s: *const FcFontSet);

  fn FcPatternGetString(
    p: *const FcPattern,
    object: *const FcChar8,
    n: c_int,
    i: &mut *const c_char,
  ) -> FcResult;

  fn FcPatternGetInteger(
    p: *const FcPattern,
    object: *const FcChar8,
    n: c_int,
    i: &mut c_int,
  ) -> FcResult;

  fn FcStrListFirst(list: *const FcStrList);
  fn FcStrListNext(list: *const FcStrList) -> *const FcChar8;
  fn FcStrListDone(list: *const FcStrList);
}

#[derive(Error, Debug)]
pub enum PlatformFontProviderErr {
  #[error("Failed to initialize font provider context: {0}")]
  Initialization(String),

  #[error("Unable to fetch the font list: {0}")]
  FontListEmpty(String),

  #[error("Unable to match the font weight: {0}")]
  FontWeightMismatch(c_int),

  #[error("Unable to match the font width: {0}")]
  FontWidthMismatch(c_int),

  #[error("Unable to fetch the font directories: {0}")]
  FontDirsEmpty(String),

  #[error(transparent)]
  InvalidString(#[from] Utf8Error),
}

type Result<T, E = PlatformFontProviderErr> = std::result::Result<T, E>;

pub struct PlatformFontProvider {
  config: *const FcConfig,
  pattern: *const FcPattern,
  object_set: *const FcObjectSet,
}

impl PlatformFontProvider {
  fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename).extension().and_then(OsStr::to_str)
  }
}

impl FontProvider for PlatformFontProvider {
  fn new() -> Result<Self> {
    let config = unsafe { FcInitLoadConfigAndFonts() };
    if config.is_null() {
      return Err(PlatformFontProviderErr::Initialization(
        "FcInitLoadConfigAndFonts failed".to_owned(),
      ));
    }

    let pattern = unsafe { FcPatternCreate() };
    if pattern.is_null() {
      return Err(PlatformFontProviderErr::Initialization("FcPatternCreate failed".to_owned()));
    }

    let object_set = unsafe {
      FcObjectSetBuild(
        FC_FILE.as_ptr(),
        FC_POSTSCRIPT_NAME.as_ptr(),
        FC_FAMILY.as_ptr(),
        FC_STYLE.as_ptr(),
        FC_WEIGHT.as_ptr(),
        FC_WIDTH.as_ptr(),
        FC_SLANT.as_ptr(),
        ptr::null::<*const FcChar8>(),
      )
    };
    if object_set.is_null() {
      return Err(PlatformFontProviderErr::Initialization("FcObjectSetBuild failed".to_owned()));
    }

    unsafe {
      FcConfigEnableHome(1);
    }

    Ok(Self { config, pattern, object_set })
  }

  fn get_api_version(&self) -> Result<usize> {
    // TODO: Implement this
    Ok(35)
  }

  fn get_all_fonts(&self) -> Result<Vec<FontDescriptor>> {
    let mut fonts: Vec<FontDescriptor> = vec![];
    let font_set: *const FcFontSet =
      unsafe { FcFontList(self.config, self.pattern, self.object_set) };

    if font_set.is_null() {
      return Err(PlatformFontProviderErr::FontListEmpty("FcFontList failed".to_owned()));
    }

    for pattern in unsafe { from_raw_parts((*font_set).fonts, (*font_set).nfont as usize) }
      .iter()
      .filter_map(|f| unsafe { f.as_ref() })
    {
      let mut path_raw: *const c_char = ptr::null();
      let mut family_raw: *const c_char = ptr::null();
      let mut style_raw: *const c_char = ptr::null();
      let mut psname_raw: *const c_char = ptr::null();
      let mut weight_raw: c_int = 0;
      let mut width_raw: c_int = 0;
      let mut slant_raw: c_int = 0;

      if unsafe { FcPatternGetString(pattern, FC_FILE.as_ptr(), 0, &mut path_raw) }
        == FcResult::Match
        && unsafe { FcPatternGetString(pattern, FC_FAMILY.as_ptr(), 0, &mut family_raw) }
          == FcResult::Match
        && unsafe { FcPatternGetString(pattern, FC_STYLE.as_ptr(), 0, &mut style_raw) }
          == FcResult::Match
        && unsafe { FcPatternGetString(pattern, FC_POSTSCRIPT_NAME.as_ptr(), 0, &mut psname_raw) }
          == FcResult::Match
        && unsafe { FcPatternGetInteger(pattern, FC_WEIGHT.as_ptr(), 0, &mut weight_raw) }
          == FcResult::Match
        && unsafe { FcPatternGetInteger(pattern, FC_WIDTH.as_ptr(), 0, &mut width_raw) }
          == FcResult::Match
        && unsafe { FcPatternGetInteger(pattern, FC_SLANT.as_ptr(), 0, &mut slant_raw) }
          == FcResult::Match
      {
        if path_raw.is_null() {
          continue;
        }

        let weight: FcWeight = FcWeight::try_from(weight_raw)?;
        let width: FcWidth = FcWidth::try_from(width_raw)?;
        let path = unsafe { CStr::from_ptr(path_raw) }.to_str()?.to_owned();

        match Self::get_extension_from_filename(path.as_str()) {
          Some("ttf") => {}
          Some("otf") => {}
          _ => continue,
        };

        let family = if family_raw.is_null() {
          "".to_owned()
        } else {
          unsafe { CStr::from_ptr(family_raw) }.to_str()?.to_owned()
        };

        let style = if style_raw.is_null() {
          weight.to_string()
        } else {
          unsafe { CStr::from_ptr(style_raw) }.to_str()?.to_owned()
        };

        let postscript = if psname_raw.is_null() {
          format!("{}-{}", family, style).to_owned()
        } else {
          unsafe { CStr::from_ptr(psname_raw) }.to_str()?.to_owned()
        };

        fonts.push(FontDescriptor {
          path: PathBuf::from(path),
          postscript,
          family,
          style,
          weight: FontWeight::from(weight),
          width: FontWidth::from(width),
          italic: slant_raw == FC_SLANT_ITALIC,
        });
      }
    }

    unsafe { FcFontSetDestroy(font_set) }

    Ok(fonts)
  }

  fn get_font_paths(&self) -> Result<Vec<PathBuf>, PlatformFontProviderErr> {
    let mut result: Vec<PathBuf> = vec![];
    let paths = unsafe { FcConfigGetFontDirs(self.config) };

    if paths.is_null() {
      return Err(PlatformFontProviderErr::FontDirsEmpty("FcConfigGetFontDirs failed".to_owned()));
    }

    unsafe { FcStrListFirst(paths) };

    loop {
      let next: *const FcChar8 = unsafe { FcStrListNext(paths) };
      if next.is_null() {
        break;
      }

      result.push(PathBuf::from(unsafe { CStr::from_ptr(next as *const c_char) }.to_str()?));
    }

    unsafe { FcStrListDone(paths) };

    Ok(result)
  }
}

impl Drop for PlatformFontProvider {
  fn drop(&mut self) {
    if !self.pattern.is_null() {
      unsafe {
        FcPatternDestroy(self.pattern);
      }
    }

    if !self.object_set.is_null() {
      unsafe {
        FcObjectSetDestroy(self.object_set);
      }
    }

    if !self.config.is_null() {
      unsafe {
        FcConfigDestroy(self.config);
      }
    }
  }
}

impl TryFrom<c_int> for FcWeight {
  type Error = PlatformFontProviderErr;
  fn try_from(value: c_int) -> Result<Self, Self::Error> {
    match value {
      0..=39 => Ok(FcWeight::Thin),
      40..=49 => Ok(FcWeight::ExtraLight),
      50..=74 => Ok(FcWeight::Light),
      75..=79 => Ok(FcWeight::Book),
      80..=99 => Ok(FcWeight::Regular),
      100..=179 => Ok(FcWeight::Medium),
      180..=199 => Ok(FcWeight::DemiBold),
      200..=204 => Ok(FcWeight::Bold),
      205..=209 => Ok(FcWeight::ExtraBold),
      210..=214 => Ok(FcWeight::Black),
      _ => {
        if value >= 215 {
          Ok(FcWeight::ExtraBlack)
        } else {
          Err(PlatformFontProviderErr::FontWeightMismatch(value))
        }
      }
    }
  }
}

impl From<FcWeight> for FontWeight {
  fn from(value: FcWeight) -> Self {
    match value {
      FcWeight::Thin => FontWeight::Thin,
      FcWeight::ExtraLight => FontWeight::ExtraLight,
      FcWeight::Light => FontWeight::Light,
      FcWeight::Book => FontWeight::Normal, // FIXME: What about that one?
      FcWeight::Regular => FontWeight::Normal,
      FcWeight::Medium => FontWeight::Medium,
      FcWeight::DemiBold => FontWeight::SemiBold,
      FcWeight::Bold => FontWeight::Bold,
      FcWeight::ExtraBold => FontWeight::ExtraBold,
      FcWeight::Black => FontWeight::Black,
      FcWeight::ExtraBlack => FontWeight::ExtraBlack,
    }
  }
}

impl TryFrom<c_int> for FcWidth {
  type Error = PlatformFontProviderErr;
  fn try_from(value: c_int) -> Result<Self, Self::Error> {
    match value {
      0..=62 => Ok(FcWidth::UltraCondensed),
      63..=74 => Ok(FcWidth::ExtraCondensed),
      75..=86 => Ok(FcWidth::Condensed),
      87..=99 => Ok(FcWidth::SemiCondensed),
      100..=112 => Ok(FcWidth::Normal),
      113..=124 => Ok(FcWidth::SemiExpanded),
      125..=149 => Ok(FcWidth::Expanded),
      150..=199 => Ok(FcWidth::ExtraExpanded),
      _ => {
        if value >= 200 {
          Ok(FcWidth::UltraExpanded)
        } else {
          Err(PlatformFontProviderErr::FontWidthMismatch(value))
        }
      }
    }
  }
}

impl From<FcWidth> for FontWidth {
  fn from(value: FcWidth) -> Self {
    match value {
      FcWidth::UltraCondensed => FontWidth::UltraCondensed,
      FcWidth::ExtraCondensed => FontWidth::ExtraCondensed,
      FcWidth::Condensed => FontWidth::Condensed,
      FcWidth::SemiCondensed => FontWidth::SemiCondensed,
      FcWidth::Normal => FontWidth::Normal,
      FcWidth::SemiExpanded => FontWidth::SemiExpanded,
      FcWidth::Expanded => FontWidth::Expanded,
      FcWidth::ExtraExpanded => FontWidth::ExtraExpanded,
      FcWidth::UltraExpanded => FontWidth::UltraExpanded,
    }
  }
}
