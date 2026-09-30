use std::{thread, time::Duration};

use langtons_ant_rust::langdons_ant::{
    ant::{Ant, AntColor},
    ant_direction::{AntDirection},
};

use langtons_ant_rust::terminal::{
    terminal_management::TerminalManagement,
};

fn main() {
    let cols: usize = 5;
    let rows: usize = 5;
    let mut ant = Ant::new(0, 0, cols, rows, AntDirection::Right);

    let mut screen_buff =  vec![vec![AntColor::new(255, 0, 0); rows]; cols];

    let mut counter = 0;

    loop {
        println!("{:?}", ant);
        if counter != cols * rows {
            println!("{:?}", ant);
            let curr_x = ant.x();
            let curr_y = ant.y();
            println!("curr_x: {curr_x}; curr_y: {curr_y}");
            let new_color = ant.make_step(&screen_buff[ant.y()][ant.x()]);
            screen_buff[curr_y][curr_x] = new_color;
            counter += 1;
        } else {
            break;
        }
    }

    let mut terminal = TerminalManagement::new();
    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_millis(500));
        terminal.draw_screen();

        if counter == 10 {
            break;
        }
        counter += 1;
    }
}
