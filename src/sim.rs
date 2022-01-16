/**
 * Represents a single hexagonal cell of the simulation
 */
#[derive(Clone, Copy, Debug)]
struct Cell {
    water: f64,
    receptive: bool,
}

/**
 * Represents a Snowflake Simulation based on
 * Reiters model,
 * see http://www.patarnott.com/pdf/SnowCrystalGrowth.pdf for
 * more details.
 */
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

    // Simulation parameters
    pub background_vapor: f64, // Alpha
    pub vapor_addition: f64,   // Beta
    pub vapor_diffusion: f64,  // Gamma
}

impl SnowflakeSim {
    pub fn new(width: usize, height: usize, alpha: f64, beta: f64, gamma: f64) -> SnowflakeSim {
        SnowflakeSim {
            current: vec![
                Cell {
                    water: 0.0,
                    receptive: false
                };
                (width + 2) * (height + 2)
            ],
            next: vec![
                Cell {
                    water: 0.0,
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
        }
    }

    /**
     * Set the water level of a cell. Useful for initial setup of the
     * seed crystal.
     */
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

    /**
     * Set the water level of a cell. Useful for initial setup of the
     * seed crystal.
     */
    pub fn get_water(&mut self, mut x: usize, mut y: usize, val: f64) -> f64 {
        // Adjust for padding manually
        x = x + 1;
        y = y + 1;
        return self.current[y * self.rwidth + x].water;
    }
    /**
     * Step the simulation one iteration
     */
    pub fn step(&mut self) {
        // Step all cells
        for y in 1..self.height + 1 {
            for x in 1..self.width + 1 {
                self.step_cell(x, y);
            }
        }

        // Loop over edge cells and introduce water to the system
        for y in 1..self.height + 1 {
            self.next[y * self.rwidth + 1].water = self.background_vapor;
            self.next[y * self.rwidth + self.width].water = self.background_vapor;
        }
        for x in 1..self.width + 1 {
            self.next[1 * self.rwidth + x].water = self.background_vapor;
            self.next[self.height * self.rwidth + x].water = self.background_vapor;
        }

        // Swap current and next
        std::mem::swap(&mut self.current, &mut self.next);
    }

    fn is_within_bounds(&self, x: isize, y: isize) -> bool {
        return x >= 1 && x <= self.width as isize && y >= 1 && y <= self.height as isize;
    }

    /**
     * Step a single cell for one iteration
     */
    fn step_cell(&mut self, x: usize, y: usize) {
        let cell: Cell = self.current[y * self.rwidth + x];
        let mut next_cell = self.next[y * self.rwidth + x];

        let mut diff_particip: f64 = 0.0;
        let mut diff_nonparticip: f64 = 0.0;

        if cell.receptive {
            diff_nonparticip = cell.water + self.vapor_addition;
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
        diff_particip = diff_particip + (self.vapor_diffusion / 2.0) * (water_avg - diff_particip);

        let started_frozen = next_cell.water >= 1.0;
        next_cell.water = diff_particip + diff_nonparticip;
        let ended_frozen = next_cell.water >= 1.0;
        if started_frozen != ended_frozen {
            // If this cell was just frozen, we need to update the neighbours as
            // receptive
            for (nx, ny) in neighbour_coords {
                if self.is_within_bounds(nx, ny) {
                    self.current[(ny as usize) * self.rwidth + (nx as usize)].receptive = true;
                }
            }
        }

        self.next[y * self.rwidth + x] = next_cell;
    }
}

/**
 * Implement display trait to allow for printing of the simulation
 */
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

// Helper methods

/**
 * Get a cell from the vector
 */
fn get_cell(cells: &mut Vec<Cell>, x: usize, y: usize, width: usize) -> &mut Cell {
    return &mut cells[y * width + x];
}

/**
 * Get an array of the 6 neighbour coordinates. These can be
 * out of bounds.
 */
fn get_neighbours(x: isize, y: isize) -> [(isize, isize); 6] {
    return if y % 2 == 0 {
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
