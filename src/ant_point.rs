#[derive(Clone, Copy)]
pub struct AntPoint {
    x: i32,
    y: i32,
}

impl AntPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn move_x(&mut self, x: i32) {
        self.x += x;
    }

    pub fn move_y(&mut self, y: i32) {
        self.y += y;
    }
}
