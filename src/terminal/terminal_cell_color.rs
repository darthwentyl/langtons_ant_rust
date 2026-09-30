#[macro_export]
macro_rules! terminal_color {
    ($r:expr, $g:expr, $b:expr) => {
        $crate::terminal::terminal_cell_color::TerminalCellColor {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TerminalCellColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl TerminalCellColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        TerminalCellColor { r, g, b }
    }

    pub fn set_fg_color(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }

    pub fn set_bg_color(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}