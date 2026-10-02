use std::io::{self, Write};

use super::terminal_cell_color::TerminalCellColor;
use super::terminal_size::TerminalSize;

pub type TerminalScreenBuff =Vec<Vec<TerminalCellColor>>;

const SHOW_CURSOR: &str = "\x1b[?25h";
const HIDE_CURSOR: &str = "\x1b[?25l";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const RESET: &str = "\x1b[0m";

pub struct TerminalManagement {
    size: TerminalSize,
    screen_buff: TerminalScreenBuff,
}

pub trait TerminalComponentDraw {
    fn draw(&mut self, buffer: &mut TerminalScreenBuff);
}

impl TerminalManagement {
    pub fn new(start_term_size: TerminalSize, default_color: TerminalCellColor) -> Self {
        TerminalManagement::enable_raw_mode();

        let cols = start_term_size.cols();
        let rows = start_term_size.rows();

        let screen_buff = vec![
                vec![default_color; cols];
                rows * 2
            ];

        Self {
            size: start_term_size,
            screen_buff: screen_buff
        }
    }

    pub fn update_terminal_state(&mut self) {
        self.size.update_terminal_size();
    }

    pub fn cols(&self) -> usize {
        self.size.cols()
    }

    pub fn rows(&self) -> usize {
        self.size.rows()
    }

    pub fn draw_elem(&mut self, item: &mut impl TerminalComponentDraw) {
        item.draw(&mut self.screen_buff);
    }

    pub fn draw_screen(&mut self) {
        let mut output = String::new();

        for ty in 0..self.size.rows() {
            output.push_str(&format!("\x1b[{};1H", ty + 1));
            for tx in 0..self.size.cols() {
                output.push_str(&format!("{}", self.screen_buff[ty * 2][tx].set_fg_color()));
                output.push_str(&format!("{}", self.screen_buff[ty * 2 + 1][tx].set_bg_color()));
                output.push('▀');
            }
        }

        output.push_str(&format!("{}", RESET));

        print!("{}", output);
        io::stdout().flush().unwrap();

    }

    fn enable_raw_mode() {
        print!("{HIDE_CURSOR}{CLEAR_SCREEN}");
        io::stdout().flush().unwrap();
    }

    fn disable_raw_mode() {
        print!("{SHOW_CURSOR}{RESET}");
        io::stdout().flush().unwrap();
    }
}

impl Drop for TerminalManagement {
    fn drop(&mut self) {
        TerminalManagement::disable_raw_mode();
    }
}
