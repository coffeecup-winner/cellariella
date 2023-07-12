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
}
