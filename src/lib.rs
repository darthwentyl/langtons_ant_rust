use std::{thread, time::Duration};

pub mod langdons_ant {
    pub mod ant;
}

pub mod terminal {
    pub mod terminal_management;
    pub mod terminal_cell_color;
    pub mod terminal_size;
}

use langdons_ant::ant::{
    Ant,
    AntColor,
    AntDirection,
};

use terminal::terminal_cell_color::TerminalCellColor;
use terminal::terminal_management::{
    TerminalManagement,
    TerminalComponentDraw,
    TerminalScreenBuff,
};

const CLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 255, 0);
const COUNTER_CLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 0, 255);

pub struct AntTermVisualization {
    terminal: TerminalManagement,
    ant: Ant,
}

impl AntTermVisualization {
    pub fn new() -> Self {
        let terminal = TerminalManagement::new(CLOCKWISE_COLOR);
        let ant = Ant::new(
            terminal.cols() / 2,
            terminal.rows(),
            terminal.cols(),
            terminal.rows() * 2,
            AntDirection::Right
        );
        Self {
            terminal: terminal,
            ant: ant,
        }
    }

    pub fn visualize(&mut self) {
        let mut counter: usize = 0;
        loop {
            thread::sleep(Duration::from_millis(10));
            self.terminal.draw_elem(&mut self.ant);
            self.terminal.draw_screen();
            if counter == 1000 {
                break;
            }
            counter += 1;
        }
    }
}

impl TerminalComponentDraw for Ant {
    fn draw(&mut self, buffer: &mut TerminalScreenBuff) {
        let curr_x = self.x();
        let curr_y = self.y();

        buffer[curr_y][curr_x] = match buffer[curr_y][curr_x] {
            CLOCKWISE_COLOR => {
                self.make_step(AntColor::Clockwise);
                COUNTER_CLOCKWISE_COLOR
            },
            COUNTER_CLOCKWISE_COLOR => {
                self.make_step(AntColor::CounterClockwise);
                CLOCKWISE_COLOR
            },
            _ => panic!("Color is not defined for algorithm: {}", buffer[curr_y][curr_x]),
        };
    }
}