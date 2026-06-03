use crate::ant_point::AntPoint;
use crate::ant_direction::EAntDirection;
use crate::ant_board_color::EAntBoardColor;

pub struct Ant {
    pos: AntPoint,
    direction: EAntDirection,
}

impl Ant {
    pub fn new(pos: AntPoint, direction: EAntDirection) -> Self {
        Self { pos, direction }
    }

    pub fn get_curr_pos(&self) -> AntPoint {
        self.pos
    }

    pub fn get_curr_direction(&self) -> EAntDirection {
        self.direction
    }

    pub fn set_position(&mut self, color: EAntBoardColor) {
        if color == EAntBoardColor::GREEN {
            match self.direction {
                EAntDirection::RIGHT => {
                    self.direction = EAntDirection::DOWN;
                    self.pos.move_y(1);
                }
                EAntDirection::DOWN => {
                    self.direction = EAntDirection::LEFT;
                    self.pos.move_x(-1);
                }
                EAntDirection::LEFT => {
                    self.direction = EAntDirection::UP;
                    self.pos.move_y(-1);
                }
                EAntDirection::UP => {
                    self.direction = EAntDirection::RIGHT;
                    self.pos.move_x(1);
                }
            }
        } else {
            match self.direction {
                EAntDirection::RIGHT => {
                    self.direction = EAntDirection::UP;
                    self.pos.move_y(-1);
                }
                EAntDirection::UP => {
                    self.direction = EAntDirection::LEFT;
                    self.pos.move_x(-1);
                }
                EAntDirection::LEFT => {
                    self.direction = EAntDirection::DOWN;
                    self.pos.move_y(1);
                }
                EAntDirection::DOWN => {
                    self.direction = EAntDirection::RIGHT;
                    self.pos.move_x(1);
                }
            }
        }
    }
}