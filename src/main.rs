mod sim;

static GRID_SIZE: usize = 400;
static ITERATIONS: usize = 1000;

fn main() {
    let mut simulation = sim::SnowflakeSim::new(GRID_SIZE, GRID_SIZE, 1.0, 0.4, 0.0001);
    simulation.set_water(GRID_SIZE / 2 - 1, GRID_SIZE / 2 - 1, 1.0);

    use std::time::Instant;
    let now = Instant::now();

    for _ in 0..ITERATIONS {
        simulation.step();
    }

    let elapsed = now.elapsed();
    println!(
        "Simulation took {:.4?} for {} iterations ({:.3?}ms per iteration)",
        elapsed,
        ITERATIONS,
        (elapsed.as_secs_f64() / ITERATIONS as f64) * 1000.0
    );
}
