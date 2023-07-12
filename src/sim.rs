use crate::{
    rules::{Cell, Neighborhood, RuleSet},
    space::Space,
};

pub struct Simulation<N: Neighborhood> {
    space: Space,
    ruleset: RuleSet<N>,
}

impl<N: Neighborhood> Simulation<N> {
    pub fn new(ruleset: RuleSet<N>) -> Self {
        Simulation {
            space: Space::new(),
            ruleset,
        }
    }

    pub fn get_cell_types_count(&self) -> usize {
        self.ruleset.cell_rules.len()
    }

    pub fn get(&self, x: i64, y: i64) -> Cell {
        self.space.get(x, y)
    }

    pub fn set(&mut self, x: i64, y: i64, cell: Cell) {
        self.space.set_curr(x, y, cell);
    }

    pub fn step(&mut self) {
        self.ruleset.apply(&mut self.space);
        self.space.finalize_step();
    }

    pub fn randomize(&mut self, x_from: i64, x_to: i64, y_from: i64, y_to: i64) {
        // TODO: Make this parameter controllable
        const DENSITY: f32 = 0.3;
        for x in x_from..x_to {
            for y in y_from..y_to {
                let new_cell = if rand::random::<f32>() <= DENSITY {
                    Cell(1 + (rand::random::<u8>() % (self.get_cell_types_count() as u8 - 1)))
                } else {
                    Cell(0)
                };
                self.space.set_curr(x, y, new_cell);
            }
        }
    }
}
