use super::*;

pub fn life() -> RuleSet {
    RuleSet::new(
        Neighborhood::Moore,
        &[
            Rule::Conditional(
                RuleCondition::CountBetween(Cell(1), 3, 3),
                Box::new(Rule::Transition(Cell(1))),
                Box::new(Rule::Static),
            ),
            Rule::Conditional(
                RuleCondition::CountBetween(Cell(1), 3, 4),
                Box::new(Rule::Static),
                Box::new(Rule::Transition(Cell(0))),
            ),
        ],
    )
}
