#[derive(Debug)]
pub enum AntColor {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug)]
pub struct Ant {
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
    direction: AntDirection,
}

impl Ant {
    pub fn new(x: usize, y: usize, x_size: usize, y_size: usize, direction: AntDirection) -> Self {
        Self { x, y, x_size, y_size, direction }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn direction(&self) -> AntDirection {
        self.direction
    }

    pub fn make_step(&mut self, color: AntColor) -> AntColor {
        let new_color = match color {
            AntColor::Clockwise => {
                self.direction.move_clockwise();
                AntColor::Counterclockwise
            }
            AntColor::Counterclockwise => {
                self.direction.move_counterclockwise();
                AntColor::Clockwise
            }
        };
        self.move_ant();
        new_color
    }

    fn move_ant(&mut self) {
        match self.direction {
            AntDirection::Up => {
                if self.y == 0 {
                    self.y = self.y_size - 1;
                } else {
                    self.y -= 1;
                }
            }
            AntDirection::Right => {
                if self.x == self.x_size - 1 {
                    self.x = 0;
                } else {
                    self.x += 1;
                }
            }
            AntDirection::Down => {
                if self.y == self.y_size - 1 {
                    self.y = 0;
                } else {
                    self.y += 1;
                }
            }
            AntDirection::Left => {
                if self.x == 0 {
                    self.x = self.x_size - 1;
                } else {
                    self.x -= 1;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AntDirection {
    Up,
    Right,
    Down,
    Left,
}

impl AntDirection {
    pub fn move_clockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        };
    }

    pub fn move_counterclockwise(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}
