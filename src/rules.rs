use crate::space::Space;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell(pub u8);

#[derive(Clone, Debug)]
pub enum RuleCondition {
    CountBetween(Cell, u8, u8),
}

impl RuleCondition {
    pub fn test(&self, x: i64, y: i64, space: &Space, neighborhood: &impl Neighborhood) -> bool {
        match self {
            RuleCondition::CountBetween(cell, from, to) => {
                let count = neighborhood.count(x, y, *cell, space);
                count >= *from && count <= *to
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Rule {
    Static,
    Transition(Cell),
    Conditional(RuleCondition, Box<Rule>, Box<Rule>),
}

impl Rule {
    pub fn apply(
        &self,
        x: i64,
        y: i64,
        cell: Cell,
        space: &Space,
        neighborhood: &impl Neighborhood,
    ) -> Cell {
        match self {
            Rule::Static => cell,
            Rule::Transition(new_cell) => *new_cell,
            Rule::Conditional(condition, rule, otherwise) => {
                if condition.test(x, y, space, neighborhood) {
                    rule.apply(x, y, cell, space, neighborhood)
                } else {
                    otherwise.apply(x, y, cell, space, neighborhood)
                }
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct RuleSet<N: Neighborhood> {
    pub cell_rules: Vec<Rule>,
    neighborhood: N,
}

impl<N: Neighborhood> RuleSet<N> {
    pub fn apply(&self, space: &mut Space) {
        // TODO: take space size and cells into account
        for x in -128..127 {
            for y in -128..127 {
                let old_cell = space.get(x, y);
                let new_cell = self.cell_rules[old_cell.0 as usize].apply(
                    x,
                    y,
                    old_cell,
                    space,
                    &self.neighborhood,
                );
                space.set_new(x, y, new_cell);
            }
        }
    }
}

pub trait Neighborhood: Default {
    fn iterate_neighbors(&self, x: i64, y: i64, f: impl FnMut(i64, i64));

    fn count(&self, x0: i64, y0: i64, cell: Cell, space: &Space) -> u8 {
        let mut sum = 0;
        self.iterate_neighbors(x0, y0, |x, y| {
            if space.get(x, y) == cell {
                sum += 1;
            }
        });
        sum
    }
}

#[derive(Default)]
pub struct MooreNeighborhood;

impl Neighborhood for MooreNeighborhood {
    fn iterate_neighbors(&self, x: i64, y: i64, mut f: impl FnMut(i64, i64)) {
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
}
