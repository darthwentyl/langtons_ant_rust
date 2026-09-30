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
