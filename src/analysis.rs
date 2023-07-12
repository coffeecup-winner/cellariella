use std::{collections::HashMap, fmt::Display};

use crate::{rules::RuleSet, sim::Simulation};

pub struct AnalysisResult {
    pub number_of_iterations: usize,
    // Count of iterations a cell N was a cell M
    pub per_cell_type_counts: Vec<Vec<f32>>,
}

impl Display for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Number of iterations: {}", self.number_of_iterations)?;
        writeln!(f, "Cell counts:")?;
        for (cell_orig, counts) in self.per_cell_type_counts.iter().enumerate() {
            if cell_orig == 0 {
                write!(f, "     \t")?;
                for i in 0..counts.len() {
                    write!(f, "{}\t", i)?;
                }
                writeln!(f)?;
            }
            write!(f, "    {}\t", cell_orig)?;
            for count in counts.iter() {
                write!(f, "{:.2}%\t", *count)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn analyze(ruleset: RuleSet) -> AnalysisResult {
    let mut number_of_iterations = 0;
    let mut per_cell_type_counts =
        vec![vec![0usize; ruleset.cell_rules.len()]; ruleset.cell_rules.len()];
    let mut totals = vec![0; ruleset.cell_rules.len()];

    const TARGET_NUMBER_OF_ITERATIONS: usize = 1000;

    const TARGET_SIMULATION_SIZE_QUADRANT: i64 = 128;

    let mut sim = Simulation::new(ruleset);
    sim.randomize(
        -TARGET_SIMULATION_SIZE_QUADRANT,
        TARGET_SIMULATION_SIZE_QUADRANT,
        -TARGET_SIMULATION_SIZE_QUADRANT,
        TARGET_SIMULATION_SIZE_QUADRANT,
    );

    let mut map = HashMap::new();

    for x in -TARGET_SIMULATION_SIZE_QUADRANT..TARGET_SIMULATION_SIZE_QUADRANT {
        for y in -TARGET_SIMULATION_SIZE_QUADRANT..TARGET_SIMULATION_SIZE_QUADRANT {
            map.insert((x, y), sim.get(x, y));
        }
    }

    for _ in 0..TARGET_NUMBER_OF_ITERATIONS {
        sim.step();
        for x in -TARGET_SIMULATION_SIZE_QUADRANT..TARGET_SIMULATION_SIZE_QUADRANT {
            for y in -TARGET_SIMULATION_SIZE_QUADRANT..TARGET_SIMULATION_SIZE_QUADRANT {
                let old_cell = map.get(&(x, y)).unwrap();
                let new_cell = sim.get(x, y);
                per_cell_type_counts[old_cell.0 as usize][new_cell.0 as usize] += 1;
                totals[old_cell.0 as usize] += 1;
            }
        }
        number_of_iterations += 1;
    }

    AnalysisResult {
        number_of_iterations,
        per_cell_type_counts: per_cell_type_counts
            .into_iter()
            .enumerate()
            .map(|(cell, counts)| {
                counts
                    .into_iter()
                    .map(|c| c as f32 / totals[cell] as f32 * 100.0)
                    .collect()
            })
            .collect(),
    }
}
