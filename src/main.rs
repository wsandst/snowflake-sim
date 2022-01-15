mod sim;

fn main() {
    let mut simulation = sim::SnowflakeSim::new(20, 20, 1.0, 0.4, 0.0001);
    simulation.set_water(20/2 - 1, 20/2 - 1, 1.0);
    for _ in 0..1 {
        simulation.step();
    }
    println!("{}", simulation);
}
