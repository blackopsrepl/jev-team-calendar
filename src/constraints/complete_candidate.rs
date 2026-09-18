use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: every task must be assigned to a discovered candidate.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .penalize(hard_weight(unary_weight))
        .named("complete_candidate")
}

fn unary_condition(entity: &Task) -> bool {
    entity.candidate_idx.is_none()
}

fn unary_weight(entity: &Task) -> HardSoftScore {
    if unary_condition(entity) {
        <HardSoftScore as Score>::one_hard()
    } else {
        <HardSoftScore as Score>::zero()
    }
}
