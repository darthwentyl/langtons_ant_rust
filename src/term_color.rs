#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TermColor {
    r: u8,
    g: u8,
    b: u8,
}

impl TermColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b
    }
}