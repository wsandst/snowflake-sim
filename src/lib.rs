use wasm_bindgen::prelude::*;
mod sim;

#[wasm_bindgen]
/**
 * Represents the simulation context which exposes an interface of the
 * simulation as well as helpers for rendering the simulation
 */
pub struct SnowflakeSimContext {
    sim: sim::SnowflakeSim,
}

#[wasm_bindgen]
impl SnowflakeSimContext {
    pub fn new(
        width: usize,
        height: usize,
        alpha: f64,
        beta: f64,
        gamma: f64,
    ) -> SnowflakeSimContext {
        SnowflakeSimContext {
            sim: sim::SnowflakeSim::new(width, height, alpha, beta, gamma),
        }
    }

    /**
     * Set the water level of a cell
     */
    pub fn set_cell(&mut self, x: usize, y: usize, water: f64) {
        self.sim.set_water(x, y, water);
    }

    /**
     * Step the Snowflake simulation one iteration
     */
    pub fn step_simulation(&mut self) {
        self.sim.step();
    }

    /**
     * Set the alpha (vapor diffusion) parameter of the
     * Snowflake Simulation
     */
    pub fn set_alpha(&mut self, value: f64) {
        self.sim.vapor_diffusion = value;
    }

    /**
     * Set the beta (background_vapor) parameter of the
     * Snowflake Simulation
     */
    pub fn set_beta(&mut self, value: f64) {
        self.sim.background_vapor = value;
    }

    /**
     * Set the gamma (vapor_addition) parameter of the
     * Snowflake Simulation
     */
    pub fn set_gamma(&mut self, value: f64) {
        self.sim.vapor_addition = value;
    }
}
