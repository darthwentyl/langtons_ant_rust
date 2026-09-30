use std::io::{self, Write};

use crate::terminal_color;
use super::terminal_cell_color::TerminalCellColor;
use super::terminal_size::TerminalSize;

type TerminalScreenBuff =Vec<Vec<TerminalCellColor>>;

const SHOW_CURSOR: &str = "\x1b[?25h";
const HIDE_CURSOR: &str = "\x1b[?25l";
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const RESET: &str = "\x1b[0m";

const COLOR_BLACK: TerminalCellColor = terminal_color!(0, 0, 0);
const COLOR_RED: TerminalCellColor = terminal_color!(255, 0, 0);
const COLOR_GREEN: TerminalCellColor = terminal_color!(0, 255, 0);

pub struct TerminalManagement {
    size: TerminalSize,
    screen_buff: TerminalScreenBuff,
}

impl TerminalManagement {
    pub fn new() -> Self {
        TerminalManagement::enable_raw_mode();

        let size = TerminalSize::new();
        let screen_buff = vec![
                vec![TerminalCellColor::new(0, 0, 0); size.cols()];
                size.rows() * 2
            ];

        TerminalManagement {
            size: size,
            screen_buff: screen_buff
        }
    }

    pub fn update_terminal_state(&mut self) {
        self.size.update_terminal_size();
    }

    pub fn draw_screen(&mut self) {
        let mut output = String::new();

        for ty in 0..self.size.rows() {
            output.push_str(&format!("\x1b[{};1H", ty + 1));
            for tx in 0..self.size.cols() {
                if self.screen_buff[ty * 2][tx] == COLOR_BLACK {
                    self.screen_buff[ty * 2][tx] = COLOR_GREEN;
                }

                if self.screen_buff[ty * 2][tx] == COLOR_GREEN {
                    self.screen_buff[ty * 2][tx] = COLOR_RED;
                } else {
                    self.screen_buff[ty * 2][tx] = COLOR_GREEN;
                }

                output.push_str(&format!("{}", self.screen_buff[ty * 2][tx].set_fg_color()));

                if self.screen_buff[ty * 2 + 1][tx] == COLOR_BLACK {
                    self.screen_buff[ty * 2 + 1][tx] = COLOR_RED;
                }
                if self.screen_buff[ty * 2 + 1][tx] == COLOR_RED {
                    self.screen_buff[ty * 2 + 1][tx] = COLOR_GREEN;
                } else {
                    self.screen_buff[ty * 2 + 1][tx] = COLOR_RED;
                }
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
        println!("TerminalManagement::droped");
    }
}
