use wasm_bindgen::prelude::*;
mod sim;

#[wasm_bindgen]
/**
 * Represents the simulation context which exposes an interface of the
 * simulation as well as helpers for rendering the simulation
 */
pub struct SnowflakeSimContext {
    sim : sim::SnowflakeSim,
}

#[wasm_bindgen]
impl SnowflakeSimContext {
    pub fn new() -> SnowflakeSimContext {
        SnowflakeSimContext { sim: sim::SnowflakeSim::new(100, 100, 1.0, 0.4, 0.0001) }
    }

    /**
     * Step the Snowflake simulation one iteration
     */
    pub fn step_simulation(&mut self)  {
        self.sim.step();
    }
}
