pub mod wireworld;

use crate::space::{Neighborhood, Space};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cell(pub u8);

#[derive(Clone, Debug)]
pub enum RuleCondition {
    CountBetween(Cell, u8, u8),
}

impl RuleCondition {
    pub fn test(&self, x: i64, y: i64, space: &Space, neighborhood: Neighborhood) -> bool {
        match self {
            RuleCondition::CountBetween(cell, from, to) => {
                let count = space.count(x, y, *cell, neighborhood);
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
        neighborhood: Neighborhood,
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

#[derive(Clone, Debug)]
pub struct RuleSet {
    pub cell_rules: Vec<Rule>,
    neighborhood: Neighborhood,
}

impl RuleSet {
    pub fn new(neighborhood: Neighborhood, rules: &[Rule]) -> Self {
        RuleSet {
            cell_rules: rules.to_vec(),
            neighborhood,
        }
    }

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
                    self.neighborhood,
                );
                space.set_new(x, y, new_cell);
            }
        }
    }
}

pub fn create_ruleset(ruleset: &str) -> Option<RuleSet> {
    match ruleset {
        "wireworld" => Some(self::wireworld::wireworld()),
        _ => None,
    }
}
