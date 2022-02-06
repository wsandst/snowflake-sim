#![feature(test)]
use wasm_bindgen::prelude::*;
mod sim;
mod sim_history;

/// Hexagon size for creating vertices. This should be
/// 1.0 and then rescaled in the view, not here in the simulation
static HEX_SIZE: f32 = 1.0;
/// At what water value should we start displaying color?
static COLOR_CUTTOFF: f32 = 0.6;

#[wasm_bindgen]

/// Represents the simulation context which exposes an interface of the
/// simulation as well as helpers for rendering the simulation
pub struct SnowflakeSimContext {
    sim: sim::SnowflakeSim,
    vertex_positions: Vec<f32>,
    vertex_colors: Vec<f32>,
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
            vertex_positions: vec![0.0; width * height * 2 * 4 * 3],
            vertex_colors: vec![0.0; width * height * 4 * 4 * 3],
        }
    }

    /// Set the water level of a cell
    pub fn set_cell(&mut self, x: usize, y: usize, water: f64) {
        self.sim.set_water(x, y, water);
    }

    /// Step the Snowflake simulation one iteration
    pub fn step_simulation(&mut self) {
        self.sim.step();
    }

    /// Create the vertex position buffer representing
    /// the hexagonal simulation
    pub fn create_vertex_positions(&mut self) {
        let mut i = 0;
        for y in 0..self.sim.height {
            for x in 0..self.sim.width {
                let (px, py) = hex_pixel_coord(x, y, HEX_SIZE);
                let corners = [
                    hex_corner(px, py, HEX_SIZE, 0),
                    hex_corner(px, py, HEX_SIZE, 1),
                    hex_corner(px, py, HEX_SIZE, 2),
                    hex_corner(px, py, HEX_SIZE, 3),
                    hex_corner(px, py, HEX_SIZE, 4),
                    hex_corner(px, py, HEX_SIZE, 5),
                ];
                // (5, 4, 0), (4, 0, 3), (0, 3, 1), (3, 1, 2) forms a hexagon
                // with 4 triangles and correct winding
                let corner_indices = [0, 1, 2, 0, 5, 2, 5, 3, 2, 5, 3, 4];
                for corner_index in corner_indices {
                    self.vertex_positions[i + 0] = corners[corner_index].0;
                    self.vertex_positions[i + 1] = corners[corner_index].1;
                    i += 2;
                }
            }
        }
    }

    /// Get the amount of vertices in the vertex position buffer
    /// for the simulation
    pub fn get_vertex_count(&self) -> usize {
        return self.sim.width * self.sim.height * 2 * 4;
    }

    /// Update the vertex color buffer based on the
    /// current state of the simulation.
    pub fn update_vertex_colors(&mut self) {
        let mut i = 0;
        for y in 0..self.sim.height {
            for x in 0..self.sim.width {
                let water = self.sim.get_water(x, y) as f32;
                let color = if water < COLOR_CUTTOFF { 0.0 } else { water };
                for _ in 0..4 * 3 {
                    self.vertex_colors[i + 0] = color;
                    self.vertex_colors[i + 1] = color;
                    self.vertex_colors[i + 2] = color;
                    self.vertex_colors[i + 3] = 1.0;
                    i += 4;
                }
            }
        }
    }

    pub fn get_vertex_positions(&self) -> js_sys::Float32Array {
        return js_sys::Float32Array::from(&self.vertex_positions[..]);
    }

    pub fn get_vertex_colors(&self) -> js_sys::Float32Array {
        return js_sys::Float32Array::from(&self.vertex_colors[..]);
    }
    
    /// Set the alpha (vapor diffusion) parameter of the Snowflake Simulation
    pub fn set_alpha(&mut self, value: f64) {
        self.sim.vapor_diffusion = value;
    }

    /// Set the beta (background_vapor) parameter of the Snowflake Simulation
    pub fn set_beta(&mut self, value: f64) {
        self.sim.background_vapor = value;
    }

    /// Set the gamma (vapor_addition) parameter of the Snowflake Simulation
    pub fn set_gamma(&mut self, value: f64) {
        self.sim.vapor_addition = value;
    }

    /// Set the alpha randomization value of the Snowflake Simulation
    /// * `range` - the percentage range of the random change of the parameter
    pub fn set_alpha_rand(&mut self, range: f64) {
        self.sim.vapor_diffusion_rand = range;
    }

    /// Set the beta randomization value of the Snowflake Simulation
    /// * `range` - the percentage range of the random change of the parameter
    pub fn set_beta_rand(&mut self, range: f64) {
        self.sim.background_vapor_rand = range;
    }

    /// Set the gamma randomization value of the Snowflake Simulation
    /// * `range` - the percentage range of the random change of the parameter
    pub fn set_gamma_rand(&mut self, range: f64) {
        self.sim.vapor_addition_rand = range;
    }

    /// Set the random seed of the simulation
    pub fn set_random_seed(&mut self, seed : u64) {
        self.sim.set_random_seed(seed);
    }
}

/// Get the floating point position of a hexagonal corner.
///
/// * `cy`, `cx` - position of the center of the hexagon
/// * `size` - size of the hexagon, from center to corner
/// * `i` - which corner to get the position for, between 0-5
fn hex_corner(cx: f32, cy: f32, hex_size: f32, i: usize) -> (f32, f32) {
    let angle_deg = (60 * (i as isize) - 30) as f32;
    let angle_rad = 3.14159265 / 180.0 * angle_deg;
    return (
        cx + hex_size * angle_rad.cos(),
        cy + hex_size * angle_rad.sin(),
    );
}

/// Get the floating point position of the center of a hexagon
///
/// * `ix`, `iy` - integer position of the hexagon
/// * `hex_size` - size of the hexagon, from center to corner
fn hex_pixel_coord(ix: usize, iy: usize, hex_size: f32) -> (f32, f32) {
    let x: f32 = ix as f32;
    let y: f32 = iy as f32;
    let rx = hex_size * (3.0 as f32).sqrt() * ((x as f32) + 0.5 * (iy % 2 == 1) as usize as f32);
    let ry = hex_size * 3.0 / 2.0 * y;
    return (rx, ry);
}
