#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RgbColor {
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl RgbColor {
  pub fn new(red: u8, green: u8, blue: u8) -> Self {
    Self { red, green, blue }
  }
}
