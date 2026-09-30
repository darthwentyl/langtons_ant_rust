mod langdons_ant {
    pub mod ant;
    pub mod ant_direction;
}

use langdons_ant::{
    ant::Ant,
    ant_direction::AntDirection,
    ant::Color,
};

fn main() {
    let cols: usize = 5;
    let rows: usize = 5;
    let mut ant = Ant::new(0, 0, cols, rows, AntDirection::Right);

    let mut screen_buff =  vec![vec![Color{r: 255, g: 0, b: 0}; rows]; cols];

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
}
