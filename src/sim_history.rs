use super::sim;

use serde::{Serialize, Deserialize};
use flate2::write::ZlibEncoder;
use flate2::write::ZlibDecoder;
use std::io::Write;

/// Track the history of an attribute
#[derive(Serialize, Deserialize)]
struct AttribHistory {
    history : Vec<(u16, f64)>
}

/// Track the history of a Snowflake Simulation
/// and allow for playback as well as
/// saving the history state as a string.
#[derive(Serialize, Deserialize)]
pub struct SimStateHistory {
    alpha_history : AttribHistory,
    beta_history: AttribHistory,
    gamma_history: AttribHistory,
    alpha_rand_history: AttribHistory,
    seed: u64,
    size: (usize, usize),
    start_filled: Vec<(usize, usize)>,
}

impl AttribHistory {
    fn new() -> AttribHistory {
        AttribHistory { history: Vec::new()}
    }

    /// Update the attribute for a certain time tick
    fn add(&mut self, tick: usize, value: f64) {
        // Only add to history if there was a change
        if self.history.len() == 0 || self.history.last().unwrap().1 != value {
            self.history.push((tick as u16, value));
        }
    }

    /// Get the attribute for a certain time tick
    fn get(&self, tick: usize) -> f64 {
        let mut first = &self.history[0];
        for second in &self.history[1..] {
            // Find which two points the tick lies imbetween,
            // then break and return the value of the first point
            if first.0 <= tick as u16 && second.0 > tick as u16 {
                break;
            }
            first = second;
        }
        // Two edgecases are also handled:
        // 1. If there is only one point, it will be returned
        // 2. If the time is past the last point, the last point will
        //    be returned
        return first.1 as f64;
    }
}

impl SimStateHistory {
    pub fn new() -> SimStateHistory {
        return SimStateHistory {
            alpha_history: AttribHistory::new(),
            beta_history: AttribHistory::new(),
            gamma_history: AttribHistory::new(),
            alpha_rand_history: AttribHistory::new(),
            seed: 0,
            size: (0,0),
            start_filled: Vec::new(),
        };
    }

    // Track/record

    /// Initiate the tracking of a simulation
    pub fn init_tracking(&mut self, sim : &sim::SnowflakeSim) {
        self.size = (sim.width, sim.height);
        self.seed = sim.seed;
        // if beta is greater than 1.0, all cells are frozen, do not record
        if sim.background_vapor < 1.0 {
            // Record the starting frozen cells
            for y in 0..sim.height {
                for x in 0..sim.width {
                    if sim.get_water(x, y) >= 1.0 {
                        self.start_filled.push((x,y));
                    }
                }
            }
        }
        self.track_tick(sim);
    }

    /// Track a simulation tick
    pub fn track_tick(&mut self, sim : &sim::SnowflakeSim) {
        let i = sim.iteration_count;
        self.alpha_history.add(i, sim.vapor_diffusion);
        self.beta_history.add(i, sim.background_vapor);
        self.gamma_history.add(i, sim.vapor_addition);
        self.alpha_rand_history.add(i, sim.vapor_diffusion_rand);
    }

    // Playback

    /// Initiate the playback of a simulation 
    pub fn init_playback(&self) -> sim::SnowflakeSim {
        let alpha = self.alpha_history.get(0);
        let beta = self.beta_history.get(0);
        let gamma = self.gamma_history.get(0);
        let mut sim = sim::SnowflakeSim::new(self.size.0, self.size.1, alpha, beta, gamma);
        sim.vapor_diffusion_rand = self.alpha_rand_history.get(0);
        sim.set_random_seed(self.seed);
        for (x, y) in &self.start_filled {
            sim.set_water(*x, *y, 1.0);
        }
        return sim;
    }

    /// Playback a simulation tick
    pub fn playback_tick(&self, sim: &mut sim::SnowflakeSim) {
        let count = sim.iteration_count;
        sim.vapor_diffusion = self.alpha_history.get(count) as f64;
        sim.background_vapor = self.beta_history.get(count) as f64;
        sim.vapor_addition = self.gamma_history.get(count) as f64;
        sim.vapor_diffusion_rand = self.alpha_rand_history.get(count) as f64;
    }

    /// Serialize the sim state history to a base64 string,
    /// useful in URLs
    pub fn serialize_to_str(&self) -> String {
        // Serialize
        let serialized_bytes = bincode::serialize(&self).unwrap();
        // Compress using flate2
        let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(&serialized_bytes).unwrap();
        let compressed_bytes = encoder.finish().unwrap();
        // Encode as base64 string
        return base64::encode_config(compressed_bytes, base64::URL_SAFE_NO_PAD);
    }

    /// Deserialize a base64 string into a sim state history
    pub fn deserialize_from_str(string : String) -> SimStateHistory {
        // Decode base64 string
        let compressed_bytes = base64::decode_config(string, base64::URL_SAFE_NO_PAD).unwrap();
        // Decompress
        let mut decoder = ZlibDecoder::new(Vec::<u8>::new());
        decoder.write_all(&compressed_bytes).unwrap();
        let serialized_bytes = decoder.finish().unwrap();
        // Deserialize
        return bincode::deserialize(&serialized_bytes).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static GRID_WIDTH: usize = 100;
    static GRID_HEIGHT: usize = 100;
    static ITERATIONS: usize = 50;

    #[test]
    fn test_history_tracking() {
        let mut sim1 = sim::SnowflakeSim::new(GRID_WIDTH, GRID_HEIGHT, 1.0, 0.4, 0.0001);
        sim1.set_water(GRID_WIDTH / 2 - 1, GRID_HEIGHT / 2 - 1, 1.0);
        sim1.set_random_seed(12831321);
    
        let mut tracker = SimStateHistory::new();
        tracker.init_tracking(&sim1);
    
        // Run simulation 1 and track the state
        for i in 0..ITERATIONS {
            sim1.step();
            if i == 8 {
                sim1.vapor_diffusion = 0.9; 
                sim1.vapor_diffusion_rand = 0.3;
            }
            else if i == 15 {
                sim1.vapor_addition = 0.01;
            }
            else if i == 32 {
                sim1.vapor_addition = 0.001;
            }
            tracker.track_tick(&sim1);
        }
    
        // Replay sim1 in sim2
        let mut sim2 = tracker.init_playback();
    
        for _ in 0..ITERATIONS {
            sim2.step();
            tracker.playback_tick(&mut sim2);
        }
    
        // Make sure sim1 and sim2 are the same
        // This means that all the parameters were reproduced
        // correctly
        assert!(compare_sims(&sim1, &sim2));
    }
    
    /// Compare two simulation states
    fn compare_sims(sim1 : &sim::SnowflakeSim, sim2: &sim::SnowflakeSim) -> bool {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let water1 = sim1.get_water(x, y);
                let water2 = sim2.get_water(x, y);
                if (water1 - water2).abs() > f64::EPSILON {
                    return false;
                }
            }
        }
        return true;
    }    
}