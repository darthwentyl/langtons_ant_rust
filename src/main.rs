use langtons_ant_rust::AntTermVisualization;

fn main() {
    {
        let mut ant = AntTermVisualization::new();
        ant.visualize().unwrap();
    }
    println!("Finished simulation :)");
}
