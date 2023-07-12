use crate::rules::Cell;

#[derive(Clone)]
pub struct Space {
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

    pub fn finalize_step(&mut self) {
        std::mem::swap(&mut self.q_ne, &mut self.new_q_ne);
        std::mem::swap(&mut self.q_se, &mut self.new_q_se);
        std::mem::swap(&mut self.q_nw, &mut self.new_q_nw);
        std::mem::swap(&mut self.q_sw, &mut self.new_q_sw);
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
}
