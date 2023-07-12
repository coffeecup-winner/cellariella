use super::*;

pub fn wireworld() -> RuleSet<MooreNeighborhood> {
    let mut ruleset = RuleSet::<MooreNeighborhood>::default();
    ruleset.cell_rules.extend_from_slice(&[
        Rule::Static,
        Rule::Transition(Cell(2)),
        Rule::Transition(Cell(3)),
        Rule::Conditional(
            RuleCondition::CountBetween(Cell(1), 1, 2),
            Box::new(Rule::Transition(Cell(1))),
            Box::new(Rule::Static),
        ),
    ]);
    ruleset
}
