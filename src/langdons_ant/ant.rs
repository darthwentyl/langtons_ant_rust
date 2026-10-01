use super::ant_direction::AntDirection;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AntColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl AntColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        AntColor { r, g, b }
    }
}

#[derive(Debug)]
pub struct Ant {
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
    direction: AntDirection,
}

const CLOCKWISE_COLOR: AntColor = AntColor{r: 0, g: 0, b: 255};
const COUNTER_CLOCKWISE_COLOR: AntColor = AntColor{r: 0, g: 255, b: 0};

impl Ant {
    pub fn new(x: usize, y: usize, x_size: usize, y_size: usize, direction: AntDirection) -> Self {
        Ant {
            x, y, x_size, y_size, direction
        }
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

    pub fn make_step(&mut self, color: &AntColor) -> AntColor {
        let new_color = match *color {
            CLOCKWISE_COLOR => {
                self.direction.move_clockwise();
                COUNTER_CLOCKWISE_COLOR
            }
            COUNTER_CLOCKWISE_COLOR => {
                self.direction.move_counterclockwise();
                CLOCKWISE_COLOR
            }
            _ =>  {
                self.direction.move_clockwise();
                CLOCKWISE_COLOR
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

