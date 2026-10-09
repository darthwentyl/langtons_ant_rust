use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct TerminalCellColor {
    r: u8,
    g: u8,
    b: u8,
}

impl TerminalCellColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn set_fg_color(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn set_bg_color(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

impl fmt::Display for TerminalCellColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}