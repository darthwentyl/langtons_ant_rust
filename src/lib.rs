use core::time;
use std::{
    sync::{
        Arc, atomic::{
            AtomicBool,
            Ordering,
        },
    },
    thread,
    error::Error,
};
use rand::random_range;

#[cfg(unix)]
use signal_hook::{
    consts::SIGINT,
    consts::SIGWINCH,
    iterator::Signals
};

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
use terminal::terminal_size::TerminalSize;
use terminal::terminal_management::{
    TerminalManagement,
    TerminalComponentDraw,
    TerminalScreenBuff,
};

const CLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 128, 0);
const COUNTERCLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 0, 128);

pub struct AntTermVisualization {
    terminal: TerminalManagement,
    ant: Ant,
}

impl AntTermVisualization {
    pub fn new() -> Self {
        let terminal_start_size = TerminalSize::new();
        let (x, y, direction, color) =
            AntTermVisualization::get_start_position(terminal_start_size.cols(), terminal_start_size.rows() * 2);

        let terminal = TerminalManagement::new(terminal_start_size, color);
        let ant = Ant::new(
            x,
            y,
            terminal.cols(),
            terminal.rows() * 2,
            direction,
        );
        Self {
            terminal: terminal,
            ant: ant,
        }
    }

    pub fn visualize(&mut self) -> Result<(), Box<dyn Error>> {
        let running = Arc::new(AtomicBool::new(true));
        let running_for_handler = Arc::clone(&running);

        let terminal_size_changed = Arc::new(AtomicBool::new(false));
        let terminal_size_changed_handler = Arc::clone(&terminal_size_changed);

        let signals = Signals::new([SIGINT, SIGWINCH]);

        thread::spawn(move ||  {
            for signal in signals.unwrap().forever() {
                if signal == SIGINT {
                    running_for_handler.store(false, Ordering::Relaxed);
                    break;
                } else if signal == SIGWINCH {
                    terminal_size_changed_handler.store(true, Ordering::Relaxed);
                }
            }
        });

        self.terminal.draw_screen();

        while running.load(Ordering::Relaxed) {
            if terminal_size_changed.swap(false, Ordering::Relaxed) {
                self.resize_screen();
                self.terminal.draw_screen();
            } else {
                self.terminal.draw_elem(&mut self.ant);
            }
            thread::sleep(time::Duration::from_millis(33));
        }
        Ok(())
    }

    fn resize_screen(&mut self) {
        self.terminal.update_terminal_state();

        let (x, y, direction, _) =
            AntTermVisualization::get_start_position(self.terminal.cols(), self.terminal.rows() * 2);
        let ant = Ant::new(
            x,
            y,
            self.terminal.cols(),
            self.terminal.rows() * 2,
            direction,
        );
        self.ant = ant;
    }

    fn get_start_position(cols: usize, rows: usize) -> (usize, usize, AntDirection, TerminalCellColor) {
        let x = random_range(0..cols);
        let y = random_range(0..rows);
        let direction = match random_range(0..4) {
            0 => AntDirection::Up,
            1 => AntDirection::Right,
            2 => AntDirection::Down,
            _ => AntDirection::Left,
        };
        let color = match random_range(0..2) {
            0 => CLOCKWISE_COLOR,
            _ => COUNTERCLOCKWISE_COLOR,
        };
        (x, y, direction, color)
    }

}

impl TerminalComponentDraw for Ant {
    fn draw(&mut self, buffer: &mut TerminalScreenBuff) {
        let curr_x = self.x();
        let curr_y = self.y();

        buffer[curr_y][curr_x] = match buffer[curr_y][curr_x] {
            CLOCKWISE_COLOR => {
                self.make_step(AntColor::Clockwise);
                COUNTERCLOCKWISE_COLOR
            },
            COUNTERCLOCKWISE_COLOR => {
                self.make_step(AntColor::Counterclockwise);
                CLOCKWISE_COLOR
            },
            _ => panic!("Color is not defined for algorithm: {}", buffer[curr_y][curr_x]),
        };

        match curr_y % 2 {
            0 => TerminalManagement::draw_screen_field(buffer, curr_x, curr_y, curr_y + 1, '▀'),
            _ => TerminalManagement::draw_screen_field(buffer, curr_x, curr_y, curr_y - 1, '▄')
        }
    }
}