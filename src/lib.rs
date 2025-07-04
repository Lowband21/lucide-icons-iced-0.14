#[cfg(feature = "iced")]
pub mod iced;
mod icon;
pub use crate::icon::Icon;
/// Bytes of the lucide font
///
/// Always use this font when relying on the icons of this crate as it may be
/// that the system installation of the font has a different version than the
/// one used by this crate
pub fn lucide_font_bytes() -> &'static [u8] {
    include_bytes!("../lucide.ttf")
}
