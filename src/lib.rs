use wasm_bindgen::prelude::*;
mod sim;

#[wasm_bindgen]
pub struct SnowflakeSimContext {
    sim : sim::SnowflakeSim,
}

#[wasm_bindgen]
impl SnowflakeSimContext {
    pub fn new() -> SnowflakeSimContext {
        SnowflakeSimContext { sim: sim::SnowflakeSim::new(100, 100, 1.0, 0.4, 0.0001) }
    }

    pub fn step_simulation(&mut self)  {
        self.sim.step();
    }
}
