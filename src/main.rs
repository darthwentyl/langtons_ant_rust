use std::{thread, time::Duration};

use langtons_ant_rust::langdons_ant::ant::{
    Ant,
    AntColor,
    AntDirection,
};

use langtons_ant_rust::terminal::terminal_cell_color::TerminalCellColor;
use langtons_ant_rust::terminal::terminal_management::{
    TerminalManagement,
    TerminalComponentDraw,
    TerminalScreenBuff,
};

struct AntDrawer(Ant);
const CLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 255, 0);
const COUNTER_CLOCKWISE_COLOR: TerminalCellColor = TerminalCellColor::new(0, 0, 255);

impl TerminalComponentDraw for AntDrawer {
    fn draw(&mut self, buffer: &mut TerminalScreenBuff) {
        let curr_x = self.0.x();
        let curr_y = self.0.y();

        buffer[curr_y][curr_x] = match buffer[curr_y][curr_x] {
            CLOCKWISE_COLOR => {
                self.0.make_step(AntColor::Clockwise);
                COUNTER_CLOCKWISE_COLOR
            },
            COUNTER_CLOCKWISE_COLOR => {
                self.0.make_step(AntColor::CounterClockwise);
                CLOCKWISE_COLOR
            },
            _ => panic!("Color is not defined for algorithm: {}", buffer[curr_y][curr_x]),
        };
    }
}

fn main() {
    let mut terminal = TerminalManagement::new(CLOCKWISE_COLOR);

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
        if counter == 1000 {
            break;
        }
        counter += 1;
    }
}
