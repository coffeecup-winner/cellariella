use super::*;

pub fn wireworld() -> RuleSet {
    RuleSet::new(
        Neighborhood::Moore,
        &[
            Rule::Static,
            Rule::Transition(Cell(2)),
            Rule::Transition(Cell(3)),
            Rule::Conditional(
                RuleCondition::CountBetween(Cell(1), 1, 2),
                Box::new(Rule::Transition(Cell(1))),
                Box::new(Rule::Static),
            ),
        ],
    )
}
