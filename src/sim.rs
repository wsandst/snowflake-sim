use oorandom::Rand64;

static RANDOM_BUFFER_SIZE: usize = 10000;
static RAND_SEED: u128 = 34917983469832;

/// Represents a single hexagonal cell of the simulation
#[derive(Clone, Copy, Debug)]
struct Cell {
    water: f64,
    receptive: bool,
}

/// Represents a Snowflake Simulation based on
/// Reiters model,
/// see http://www.patarnott.com/pdf/SnowCrystalGrowth.pdf for
/// more details.
#[derive(Debug)]
pub struct SnowflakeSim {
    // Simulation state
    current: Vec<Cell>,
    next: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    // Real width and height (the array is padded)
    rwidth: usize,
    rheight: usize,

    // Random buffer
    random_buffer: Vec<f64>,
    random_buffer_index: usize,

    // Simulation parameters
    /// Alpha
    pub vapor_diffusion: f64,
    /// Beta
    pub background_vapor: f64,
    /// Gamma
    pub vapor_addition: f64,
    // Randomization of simulation parameters in percent
    pub background_vapor_rand : f64,
    pub vapor_addition_rand : f64,
    pub vapor_diffusion_rand : f64,
    
}

impl SnowflakeSim {
    pub fn new(width: usize, height: usize, alpha: f64, beta: f64, gamma: f64) -> SnowflakeSim {
        let mut sim = SnowflakeSim {
            current: vec![
                Cell {
                    water: beta,
                    receptive: false
                };
                (width + 2) * (height + 2)
            ],
            next: vec![
                Cell {
                    water: beta,
                    receptive: false
                };
                (width + 2) * (height + 2)
            ],
            width: width,
            height: height,
            rwidth: width + 2,
            rheight: height + 2,
            vapor_diffusion: alpha,
            background_vapor: beta,
            vapor_addition: gamma,
            vapor_diffusion_rand: 0.0,
            background_vapor_rand: 0.0,
            vapor_addition_rand: 0.0,
            random_buffer: vec![0.0; RANDOM_BUFFER_SIZE],
            random_buffer_index: 0,
        };
        // Setup the random buffer which is used to improve performance of
        // random numbers
        sim.set_random_seed(RAND_SEED);
        return sim;
    }

    /// Set the water level of a cell. Useful for initial setup of the
    /// seed crystal.
    pub fn set_water(&mut self, mut x: usize, mut y: usize, val: f64) {
        // Adjust for padding manually
        x = x + 1;
        y = y + 1;
        self.current[y * self.rwidth + x].water = val;
        if val >= 1.0 {
            // This cell is now frozen, we have to do
            // some bookkeeping and mark neighbours as receptive
            self.current[y * self.rwidth + x].receptive = true;
            self.next[y * self.rwidth + x].receptive = true;
            let neighbour_coords = get_neighbours(x as isize, y as isize);
            for (nx, ny) in neighbour_coords {
                if self.is_within_bounds(nx, ny) {
                    self.current[(ny as usize) * self.rwidth + (nx as usize)].receptive = true;
                    self.next[(ny as usize) * self.rwidth + (nx as usize)].receptive = true;
                }
            }
        }
    }

    pub fn set_random_seed(&mut self, seed : u128) {
        let mut rand = Rand64::new(seed);
        for i in 0..self.random_buffer.len() {
            self.random_buffer[i] = rand.rand_float();
        }
    }

    fn get_background_vapor(&mut self) -> f64 {
        if self.background_vapor_rand > 0.0 {
            return self.background_vapor * self.get_random_factor(self.background_vapor_rand);
        }
        else {
            return self.background_vapor;
        }
    }

    fn get_vapor_diffusion(&mut self) -> f64 {
        if self.vapor_diffusion_rand > 0.0 {
            return self.vapor_diffusion * self.get_random_factor(self.vapor_diffusion_rand);
        }
        else {
            return self.vapor_diffusion;
        }
    }

    fn get_vapor_addition(&mut self) -> f64 {
        if self.vapor_addition_rand > 0.0 {
            // Randomize by a factor of the random vapor addition param
            return self.vapor_addition * self.get_random_factor(self.vapor_addition_rand);
        }
        else {
            return self.vapor_addition;
        }
    }

    /// Get the next random f64 from the random buffer.
    /// 
    /// Wraps around after `RANDOM_BUFFER_SIZE` has been exceeded
    fn get_next_rand(&mut self) -> f64 {
        let val = self.random_buffer[self.random_buffer_index % RANDOM_BUFFER_SIZE];
        self.random_buffer_index += 1;
        return val;
    }

    /// Get a random number mapped between (1 - rand_range, 1 + rand_range),
    /// with negative values clamped to 0.
    fn get_random_factor(&mut self, rand_range : f64) -> f64 {
        // Random number mapped between (1 - rand_range, 1 + rand_range)
        // Clamp to 0 to prevent negative values
        return (0.0 as f64).max(1.0 + rand_range * (1.0 - self.get_next_rand() * 2.0));
    }


    /// Get the water level of a cell.
    pub fn get_water(&mut self, mut x: usize, mut y: usize) -> f64 {
        // Adjust for padding manually
        x = x + 1;
        y = y + 1;
        return self.current[y * self.rwidth + x].water;
    }

    /// Step the Reiters Model simulation one iteration.
    pub fn step(&mut self) {
        // Step all cells
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.step_cell(x, y);
            }
        }

        // Loop over edge cells and introduce water to the system
        for y in 1..self.height + 1 {
            self.next[y * self.rwidth + 1].water = self.get_background_vapor();
            self.next[y * self.rwidth + self.width].water = self.get_background_vapor();
        }
        for x in 1..self.width + 1 {
            self.next[1 * self.rwidth + x].water = self.get_background_vapor();
            self.next[self.height * self.rwidth + x].water = self.get_background_vapor();
        }

        // Swap current and next
        std::mem::swap(&mut self.current, &mut self.next);
    }

    /// Is a position within bounds of the simulation?
    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        return x >= 1 && x <= self.width as isize && y >= 1 && y <= self.height as isize;
    }

    /// Step a single cell for one iteration
    fn step_cell(&mut self, x: usize, y: usize) {
        let cell: Cell = self.current[y * self.rwidth + x];
        let mut next_cell = self.next[y * self.rwidth + x];

        let mut diff_particip: f64 = 0.0;
        let mut diff_nonparticip: f64 = 0.0;

        if cell.receptive {
            diff_nonparticip = cell.water + self.get_vapor_addition();
        } else {
            diff_particip = cell.water;
        }

        // Count the average water content among the neighbours
        let mut water_avg: f64 = 0.0;

        let neighbour_coords = get_neighbours(x as isize, y as isize);
        for (nx, ny) in neighbour_coords {
            let neighbour = self.current[((ny) as usize) * self.rwidth + nx as usize];
            if !neighbour.receptive {
                water_avg += neighbour.water;
            }
        }

        water_avg = water_avg / 6.0;

        // Diffuse
        diff_particip = diff_particip + (self.get_vapor_diffusion() / 2.0) * (water_avg - diff_particip);

        let started_frozen = next_cell.water >= 1.0;
        next_cell.water = diff_particip + diff_nonparticip;
        let ended_frozen = next_cell.water >= 1.0;
        if started_frozen != ended_frozen {
            // If this cell was just frozen, we need to update the neighbours as
            // receptive
            for (nx, ny) in neighbour_coords {
                if self.is_within_bounds(nx, ny) {
                    self.next[(ny as usize) * self.rwidth + (nx as usize)].receptive = true;
                }
            }
        }

        self.next[y * self.rwidth + x] = next_cell;
    }
}

// Implement display trait to allow for printing of the simulation
impl std::fmt::Display for SnowflakeSim {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.rheight {
            for x in 0..self.rheight {
                fmt.write_str(&format!("{:.2} ", self.current[y * self.rwidth + x].water))?;
            }
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}

/// Get an array of the 6 neighbour coordinates. These can be
/// out of bounds.
fn get_neighbours(x: isize, y: isize) -> [(isize, isize); 6] {
    return if y % 2 == 1 {
        [
            (x + 1, y),
            (x, y - 1),
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
        ]
    } else {
        [
            (x + 1, y),
            (x + 1, y - 1),
            (x, y - 1),
            (x - 1, y),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    };
}
