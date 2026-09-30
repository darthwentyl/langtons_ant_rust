use std::io::{self};

pub struct TerminalSize {
    rows: usize,
    cols: usize,
}

impl TerminalSize {
    pub fn new() -> Self {
        let mut terminal_size = TerminalSize {
            rows: 0,
            cols: 0
        };
        terminal_size.update_terminal_size();
        terminal_size
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn update_terminal_size(&mut self) {
        let mut size = libc::winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        let result = unsafe {
            libc::ioctl(
                libc::STDOUT_FILENO,
                libc::TIOCGWINSZ,
                &mut size
            )
        };

        if result == -1 {
            panic!("Cannot get terminal size: {}", io::Error::last_os_error());
        } else {
            self.cols = size.ws_col as usize;
            self.rows = size.ws_row as usize;
        }
    }
}
