use crate::rules::Cell;

#[derive(Clone, Copy, Debug)]
pub enum Neighborhood {
    Moore,
    VonNeumann,
}

#[derive(Clone)]
pub struct Space {
    current_step: u64,
    // 4 space quadrants
    q_ne: Vec<Cell>,
    q_se: Vec<Cell>,
    q_nw: Vec<Cell>,
    q_sw: Vec<Cell>,
    // New values
    new_q_ne: Vec<Cell>,
    new_q_se: Vec<Cell>,
    new_q_nw: Vec<Cell>,
    new_q_sw: Vec<Cell>,
}

// TODO: growable space
const BLOCK_SIZE: usize = 128;
const SPACE_SIZE: usize = BLOCK_SIZE * BLOCK_SIZE;

impl Space {
    pub fn new() -> Self {
        Space {
            current_step: 0,
            q_ne: vec![Cell(0); SPACE_SIZE],
            q_se: vec![Cell(0); SPACE_SIZE],
            q_nw: vec![Cell(0); SPACE_SIZE],
            q_sw: vec![Cell(0); SPACE_SIZE],
            new_q_ne: vec![Cell(0); SPACE_SIZE],
            new_q_se: vec![Cell(0); SPACE_SIZE],
            new_q_nw: vec![Cell(0); SPACE_SIZE],
            new_q_sw: vec![Cell(0); SPACE_SIZE],
        }
    }

    pub fn current_step(&self) -> u64 {
        self.current_step
    }

    pub fn finalize_step(&mut self) {
        std::mem::swap(&mut self.q_ne, &mut self.new_q_ne);
        std::mem::swap(&mut self.q_se, &mut self.new_q_se);
        std::mem::swap(&mut self.q_nw, &mut self.new_q_nw);
        std::mem::swap(&mut self.q_sw, &mut self.new_q_sw);
        self.current_step += 1;
    }

    #[allow(clippy::collapsible_else_if)]
    pub fn get(&self, x: i64, y: i64) -> Cell {
        let x = x % BLOCK_SIZE as i64;
        let y = y % BLOCK_SIZE as i64;
        if x >= 0 {
            if y >= 0 {
                self.q_ne[x as usize + y as usize * BLOCK_SIZE]
            } else {
                self.q_se[x as usize + (-y) as usize * BLOCK_SIZE]
            }
        } else {
            if y >= 0 {
                self.q_nw[(-x) as usize + y as usize * BLOCK_SIZE]
            } else {
                self.q_sw[(-x) as usize + (-y) as usize * BLOCK_SIZE]
            }
        }
    }

    #[allow(clippy::collapsible_else_if)]
    pub fn set_new(&mut self, x: i64, y: i64, cell: Cell) {
        let x = x % BLOCK_SIZE as i64;
        let y = y % BLOCK_SIZE as i64;
        if x >= 0 {
            if y >= 0 {
                self.new_q_ne[x as usize + y as usize * BLOCK_SIZE] = cell;
            } else {
                self.new_q_se[x as usize + (-y) as usize * BLOCK_SIZE] = cell;
            }
        } else {
            if y >= 0 {
                self.new_q_nw[(-x) as usize + y as usize * BLOCK_SIZE] = cell;
            } else {
                self.new_q_sw[(-x) as usize + (-y) as usize * BLOCK_SIZE] = cell;
            }
        }
    }

    #[allow(clippy::collapsible_else_if)]
    pub fn set_curr(&mut self, x: i64, y: i64, cell: Cell) {
        let x = x % BLOCK_SIZE as i64;
        let y = y % BLOCK_SIZE as i64;
        if x >= 0 {
            if y >= 0 {
                self.q_ne[x as usize + y as usize * BLOCK_SIZE] = cell;
            } else {
                self.q_se[x as usize + (-y) as usize * BLOCK_SIZE] = cell;
            }
        } else {
            if y >= 0 {
                self.q_nw[(-x) as usize + y as usize * BLOCK_SIZE] = cell;
            } else {
                self.q_sw[(-x) as usize + (-y) as usize * BLOCK_SIZE] = cell;
            }
        }
    }

    pub fn count(&self, x0: i64, y0: i64, cell: Cell, neighborhood: Neighborhood) -> u8 {
        let mut sum = 0;
        self.iterate_neighbors(neighborhood, x0, y0, |x, y| {
            if self.get(x, y) == cell {
                sum += 1;
            }
        });
        sum
    }

    pub fn iterate_neighbors(
        &self,
        neighborhood: Neighborhood,
        x: i64,
        y: i64,
        f: impl FnMut(i64, i64),
    ) {
        match neighborhood {
            Neighborhood::Moore => self.iterate_neighbors_moore(x, y, f),
            Neighborhood::VonNeumann => self.iterate_neighbors_von_neumann(x, y, f),
        }
    }

    fn iterate_neighbors_moore(&self, x: i64, y: i64, mut f: impl FnMut(i64, i64)) {
        f(x - 1, y - 1);
        f(x, y - 1);
        f(x + 1, y - 1);
        f(x - 1, y);
        f(x, y);
        f(x + 1, y);
        f(x - 1, y + 1);
        f(x, y + 1);
        f(x + 1, y + 1);
    }

    fn iterate_neighbors_von_neumann(&self, x: i64, y: i64, mut f: impl FnMut(i64, i64)) {
        f(x, y - 1);
        f(x + 1, y);
        f(x, y + 1);
        f(x - 1, y);
    }
}
