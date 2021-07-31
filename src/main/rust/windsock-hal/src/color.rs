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

    pub fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xff) as u8,
            green: ((hex >> 8) & 0xff) as u8,
            blue: (hex & 0xff) as u8,
        }
    }
}
