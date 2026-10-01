use std::{thread, time::Duration};

use langtons_ant_rust::{
    langdons_ant::{
        ant::{Ant, AntColor},
        ant_direction::AntDirection,
    },
    terminal_color
};

use langtons_ant_rust::terminal::{
    terminal_management::TerminalManagement,
    terminal_management::TerminalComponentDraw,
    terminal_management::TerminalScreenBuff,
};

struct AntDrawer(Ant);

impl TerminalComponentDraw for AntDrawer {
    fn draw(&mut self, buffer: &mut TerminalScreenBuff) {
        let curr_x = self.0.x();
        let curr_y = self.0.y();
        let cur_color = AntColor::new(
            buffer[curr_y][curr_x].r,
            buffer[curr_y][curr_x].g,
            buffer[curr_y][curr_x].b
        );
        let cur_color = self.0.make_step(&cur_color);
        buffer[curr_y][curr_x] = terminal_color!(
            cur_color.r, cur_color.g, cur_color.b
        );
    }
}

fn main() {
    let mut terminal = TerminalManagement::new();

    let ant = Ant::new(
        terminal.cols() / 2, terminal.rows(),
        terminal.cols(), terminal.rows() * 2,
        AntDirection::Right
    );
    let mut ant_drawer = AntDrawer(ant);

    let mut counter: usize = 0;
    loop {
        thread::sleep(Duration::from_millis(10));
        terminal.draw_elem(&mut ant_drawer);
        terminal.draw_screen();
        if counter == 10000 {
            break;
        }
        counter += 1;
    }
}
