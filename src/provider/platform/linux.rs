use crate::provider::FontProvider;

use std::os::raw::c_char;

#[repr(C)]
struct FcConfig {
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
  _private: [u8; 0],
}

#[link(name = "fontconfig")]
extern "C" {
  fn FcInitLoadConfig() -> *const FcConfig;

  fn FcPatternCreate() -> *const FcPattern;

  fn FcObjectSetBuild(first: *const c_char, ...) -> *const FcObjectSet;

  fn FcFontList(
    config: *const FcConfig,
    p: *const FcPattern,
    os: *const FcObjectSet,
  ) -> *const FcFontSet;
}

pub struct LinuxFontProvider {
  config: *const FcConfig,
}

impl LinuxFontProvider {
  fn new() -> Self {
    Self { config: unsafe { FcInitLoadConfig() } }
  }
}

impl FontProvider for LinuxFontProvider {
  fn get_all_fonts(&self) {}
}

async fn test() {
  let _x = LinuxFontProvider::new();
}
