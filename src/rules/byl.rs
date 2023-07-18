use crate::sim::Simulation;

use super::*;

// Implementation based on https://www.asa3.org/ASA/PSCF/1989/PSCF3-89Byl.html

pub fn byl() -> RuleSet {
    RuleSet::new(
        Neighborhood::VonNeumann,
        &[
            Rule::TryCustom(byl_transition_table, Box::new(Rule::Static)),
            Rule::TryCustom(byl_transition_table, Box::new(Rule::Static)),
            Rule::TryCustom(byl_transition_table, Box::new(Rule::Static)),
            Rule::TryCustom(
                byl_transition_table,
                Box::new(Rule::Custom(get_next_cw_from_smallest)),
            ),
            Rule::TryCustom(
                byl_transition_table,
                Box::new(Rule::Custom(get_next_cw_from_smallest)),
            ),
            Rule::TryCustom(byl_transition_table, Box::new(Rule::Static)),
            Rule::TryCustom(
                byl_transition_table,
                Box::new(Rule::Custom(get_next_cw_from_smallest)),
            ),
        ],
    )
}

// Custom rules for Byl loop, does not cover everything -
// defaults to the other rules.
fn byl_transition_table(
    x: i64,
    y: i64,
    cell: Cell,
    space: &Space,
    neighborhood: Neighborhood,
) -> Option<Cell> {
    let mut neighbors = vec![];
    space.iterate_neighbors(neighborhood, x, y, |x, y| {
        let mut curr = space.get(x, y).0;
        if curr == 0 {
            curr = 7;
        }
        neighbors.push(curr);
    });
    // CW -> CCW
    neighbors.reverse();
    let mut smallest = neighbors[0];
    let mut idx = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 1..=3 {
        if neighbors[i] < smallest {
            smallest = neighbors[i];
            idx = i;
        }
    }
    neighbors.rotate_left(idx);
    match (
        cell.0,
        neighbors[0],
        neighbors[1],
        neighbors[2],
        neighbors[3],
    ) {
        (0, 1, 7, 7, 6) => Some(2),
        (0, 2, 5, 7, 7) => Some(5),
        (0, 2, 7, 7, 3) => Some(2),
        (0, 2, 7, 7, 4) => Some(2),
        (0, 2, 7, 7, 6) => Some(6),
        (0, 3, 5, 7, 7) => Some(2),
        (0, 3, 6, 7, 7) => Some(3),
        // NOTE: this rule incorrectly returns 6 in the article
        (0, 3, 7, 7, 6) => Some(5),
        (0, 6, 7, 7, 7) => Some(3),
        (1, 2, 5, 2, 7) => Some(0),
        // NOTE: rotated rule
        (1, 2, 7, 2, 5) => Some(0),
        (2, 2, 2, 7, 7) => Some(0),
        // NOTE: rotated rule
        (2, 2, 7, 7, 2) => Some(0),
        (2, 2, 3, 5, 7) => Some(5),
        (2, 2, 5, 2, 7) => Some(5),
        // NOTE: rotated rule
        (2, 2, 7, 2, 5) => Some(5),
        (2, 2, 6, 5, 7) => Some(6),
        (2, 2, 7, 2, 7) => Some(0),
        (2, 3, 5, 7, 7) => Some(5),
        (2, 7, 7, 7, 7) => Some(0),
        (3, 4, 6, 7, 7) => Some(3),
        (3, 4, 7, 7, 7) => Some(0),
        (3, 6, 7, 7, 7) => Some(6),
        (3, 7, 7, 7, 7) => Some(0),
        (4, 2, 3, 6, 5) => Some(6),
        (4, 2, 5, 7, 6) => Some(5),
        (5, 1, 7, 2, 4) => Some(2),
        (5, 2, 2, 2, 7) => Some(0),
        // NOTE: rotated rule
        (5, 2, 2, 7, 2) => Some(0),
        // NOTE: rotated rule
        (5, 2, 7, 2, 2) => Some(0),
        (5, 2, 2, 7, 6) => Some(0),
        // NOTE: rotated rule
        (5, 2, 7, 6, 2) => Some(0),
        (5, 2, 3, 2, 7) => Some(1),
        // NOTE: rotated rule
        (5, 2, 7, 2, 3) => Some(1),
        (5, 2, 4, 2, 6) => Some(2),
        // NOTE: rotated rule
        (5, 2, 6, 2, 4) => Some(2),
        (5, 2, 4, 7, 7) => Some(2),
        (5, 2, 7, 7, 4) => Some(2),
        (5, 3, 7, 7, 6) => Some(2),
        (6, 1, 5, 2, 3) => Some(5),
        (6, 1, 7, 7, 7) => Some(2),
        (6, 2, 6, 3, 5) => Some(3),
        (6, 2, 6, 5, 3) => Some(5),
        (6, 2, 6, 5, 7) => Some(6),
        _ => None,
    }
    .map(Cell)
}

// Pick the neighbor CW from the smallest neighbor.
// If there are several smallest one, pick the one
// CW from the last of them.
fn get_next_cw_from_smallest(
    x: i64,
    y: i64,
    _cell: Cell,
    space: &Space,
    neighborhood: Neighborhood,
) -> Cell {
    let mut smallest = 7;
    let mut pick_next = true;
    let mut result = smallest;
    // Assume iteration is CW
    space.iterate_neighbors(neighborhood, x, y, |x, y| {
        let mut curr = space.get(x, y).0;
        // State 0 is actually state 7 in Byl, but this is only needed
        // to calculate the smallest neighbor, so fake it here.
        if curr == 0 {
            curr = 7;
        }
        if pick_next {
            result = curr;
            pick_next = false;
        }
        if curr <= smallest {
            smallest = curr;
            pick_next = true;
        }
    });
    if pick_next {
        // The last neighbor is the smallest, do another neighbor loop
        // to find the first non-smallest one.
        space.iterate_neighbors(neighborhood, x, y, |x, y| {
            let mut curr = space.get(x, y).0;
            if curr == 0 {
                curr = 7;
            }
            if curr == smallest {
                return;
            }
            if pick_next {
                result = curr;
                pick_next = false;
            }
        });
    }
    if result == 7 {
        result = 0;
    }
    Cell(result)
}

pub fn create_initial_state(sim: &mut Simulation) {
    sim.set(1, 0, Cell(2));
    sim.set(2, 0, Cell(2));
    sim.set(0, 1, Cell(2));
    sim.set(1, 1, Cell(6));
    sim.set(2, 1, Cell(3));
    sim.set(3, 1, Cell(2));
    sim.set(0, 2, Cell(2));
    sim.set(1, 2, Cell(6));
    sim.set(2, 2, Cell(4));
    sim.set(3, 2, Cell(2));
    sim.set(1, 3, Cell(2));
    sim.set(2, 3, Cell(5));
}
