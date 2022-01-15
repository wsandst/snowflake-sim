
#[derive(Clone, Copy, Debug)]
struct Cell {
    water : f64,
    receptive : bool,
}

#[derive(Debug)]
pub struct SnowflakeSim {
    // Simulation state
    current : Vec<Cell>,
    next : Vec<Cell>,
    width : usize,
    height : usize,
    // Simulation parameters
    background_vapor : f64, // Alpha
    vapor_addition : f64, // Beta
    vapor_diffusion  : f64, // Gamma
}

impl SnowflakeSim {
    pub fn new(width: usize, height: usize, alpha: f64, beta: f64, gamma: f64) -> SnowflakeSim {
        SnowflakeSim { 
            current: vec![Cell {water: 0.0, receptive: false}; width*height],
            next : vec![Cell {water: 0.0, receptive: false}; width*height],
            width : width,
            height : height,
            vapor_diffusion  : alpha,
            background_vapor : beta,
            vapor_addition : gamma,
        }
    }

    pub fn set_water(&mut self, x : usize, y: usize, val : f64) {
        self.current[y*self.width + x].water = val;
        if val >= 1.0 {
            self.current[y*self.width + x].receptive = true;
            self.next[y*self.width + x].receptive = true;
            for (nx, ny) in get_neighbours(x as isize, y as isize, self.width, self.height) {
                self.current[(ny as usize)*self.width + (nx as usize)].receptive = true;
                self.next[(ny as usize)*self.width + (nx as usize)].receptive = true;
            }
        }
    }
    
    pub fn step(&mut self) {
        // Step all cells
        for y in 0..self.height {
            for x in 0..self.width {
                self.step_cell(x, y);
            }
        }

        // Loop over edge cells and introduce water to the system
        for y in 0..self.height {
            get_cell(&mut self.next, 0, y, self.width).water = self.background_vapor;
            get_cell(&mut self.next, self.width-1, y, self.width).water = self.background_vapor;
        }
        for x in 0..self.width {
            get_cell(&mut self.next, x, 0, self.width).water = self.background_vapor;
            get_cell(&mut self.next, x, self.height-1, self.width).water = self.background_vapor;
        }

        // Swap current and next
        std::mem::swap(&mut self.current, &mut self.next);
    }

    fn step_cell(&mut self, x : usize, y: usize) {
        let cell : Cell = self.current[y*self.width + x];
        let mut next_cell = self.next[y*self.width + x]; //&Cell = &mut self.next[y*self.width + x];

        let mut diff_particip : f64 = 0.0;
        let mut diff_nonparticip : f64 = 0.0;

        if cell.receptive {
            diff_nonparticip = cell.water + self.vapor_addition;
        }
        else {
            diff_particip = cell.water;
        }

        let mut water_avg : f64 = 0.0;
        for (nx, ny) in get_neighbours(x as isize, y as isize, self.width, self.height) {
            let neighbour = self.current[(ny as usize)*self.width + nx as usize];
            if !neighbour.receptive {
                water_avg += neighbour.water;
            }
        }
        water_avg = water_avg / 6.0;

        diff_particip = diff_particip + (self.vapor_diffusion/2.0) * (water_avg - diff_particip);

        let started_frozen = next_cell.water >= 1.0;
        next_cell.water = diff_particip + diff_nonparticip;
        let ended_frozen = next_cell.water >= 1.0;
        if started_frozen != ended_frozen {
            for (nx, ny) in get_neighbours(x as isize, y as isize, self.width, self.height) {
                self.current[(ny as usize)*self.width + (nx as usize)].receptive = true;
            }
        }
        self.next[y*self.width + x] = next_cell;
    }
}

impl std::fmt::Display for SnowflakeSim {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.height {
                fmt.write_str(&format!("{:.2} ", self.current[y*self.width+x].water))?;
            }
            fmt.write_str("\n")?;
        }
        Ok(())
    }
}

fn get_cell(cells : &mut Vec<Cell>, x : usize, y: usize, width : usize) -> &mut Cell {
    return &mut cells[y*width + x];
}

/**
 * Get iterator of neighbour coordinates in an
 * offset based hex coordinate system
 */
fn get_neighbours(x: isize, y: isize, width: usize, height : usize) -> impl Iterator<Item = (isize, isize)> {
    let neighbours : std::vec::Vec<(isize, isize)> = 
        if y % 2 == 0 { // Even 
            vec![(1,  0), (0, -1), (-1, -1), (-1,  0), (-1, 1), (0, 1)]
        }
        else { // Odd
            vec![(1,  0), (1, -1), (0, -1), (-1,  0), (0, 1), (1, 1)]
        };
    neighbours.into_iter()
        .map(move |(nx, ny)| (nx + x , ny + y))
        .filter( move
            |(nx, ny)| nx >= &0 && nx < &(width as isize) && ny >= &0 && ny < &(height as isize))
}