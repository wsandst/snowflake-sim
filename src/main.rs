mod sim;

static GRID_WIDTH: usize = 500;
static GRID_HEIGHT: usize = 500;
static ITERATIONS: usize = 1000;

fn main() {
    let mut simulation = sim::SnowflakeSim::new(GRID_WIDTH, GRID_HEIGHT, 1.0, 0.4, 0.0001);
    simulation.set_water(GRID_WIDTH / 2 - 1, GRID_HEIGHT / 2 - 1, 1.0);

    use std::time::Instant;
    let now = Instant::now();

    for _ in 0..ITERATIONS {
        simulation.step();
    }

    let elapsed = now.elapsed();
    println!(
        "Simulation took {:.3?} for {} iterations ({:.3?} per iteration)",
        elapsed,
        ITERATIONS,
        elapsed.div_f64(ITERATIONS as f64)
    );
}
