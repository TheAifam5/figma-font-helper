#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
#[cfg_attr(target_os = "windows", path = "platform/windows.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
mod fontprovider;

pub use fontprovider::{PlatformFontProvider, PlatformFontProviderErr};