cfg_if::cfg_if! {
  if #[cfg(target_os = "linux")] {
    mod linux;
    pub use linux::{LinuxFontProvider as FontProvider, LinuxFontProviderErr as FontProviderErr};
  } else if #[cfg(target_os = "windows")] {
    mod windows;
    pub use windows::{WindowsFontProvider as FontProvider, WindowsFontProviderErr as FontProviderErr};
  } else if #[cfg(target_os = "macos")] {
    mod windows;
    pub use windows::{MacOSFontProvider as FontProvider, MacOSFontProviderErr as FontProviderErr};
  } else {
    panic!("Platform not supported");
  }
}
