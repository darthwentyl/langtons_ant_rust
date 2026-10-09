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
            screen_buff: screen_buff,
        }
    }

    pub fn update_terminal_state(&mut self) {
        let old_cols = self.size.cols();
        let old_rows = self.size.rows();

        let new_size = TerminalSize::new();
        self.size = new_size;

        let red = TerminalCellColor::new(255, 0, 0);
        let mut new_screen_buff = vec![
                vec![red; self.size.cols()];
                self.size.rows() * 2
            ];

        for y in 0..(self.size.rows() * 2) {
            for x in 0..self.size.cols() {
                let old_x = x * old_cols / self.size.cols();
                let old_y = y * old_rows / self.size.rows();
                new_screen_buff[y][x] = self.screen_buff[old_y][old_x];
            }
        }

        self.screen_buff = new_screen_buff;

        let mut stdout = io::stdout().lock();
        stdout.write_all(CLEAR_SCREEN.as_bytes()).unwrap();
        stdout.flush().unwrap();

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

    pub fn draw_screen(&self) {
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

        let mut stdout = io::stdout().lock();
        stdout.write_all(output.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }

    pub fn draw_screen_field(buffer: &mut TerminalScreenBuff, x: usize, y: usize, next_y: usize, block_char: char) {
        let mut stdout = io::stdout().lock();
        write!(
                stdout,
                "\x1b[{};{}H{}{}{}{}",
                y / 2,
                x,
                buffer[y][x].set_fg_color(),
                buffer[next_y][x].set_bg_color(),
                block_char,
                RESET
            ).unwrap();
        stdout.flush().unwrap();
    }

    fn enable_raw_mode() {
        let mut stdout = io::stdout().lock();
        write!(stdout,
            "{}{}",
            HIDE_CURSOR,
            CLEAR_SCREEN)
        .unwrap();
        stdout.flush().unwrap();
    }

    fn disable_raw_mode(rows: usize) {
        let mut stdout = io::stdout().lock();
        write!(stdout,
            "\x1b[{};1H{}{}\n",
            rows,
            SHOW_CURSOR,
            RESET)
        .unwrap();
        stdout.flush().unwrap();
    }
}

impl Drop for TerminalManagement {
    fn drop(&mut self) {
        self.draw_screen();
        TerminalManagement::disable_raw_mode(self.rows());
    }
}
