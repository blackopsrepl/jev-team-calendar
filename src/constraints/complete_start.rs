use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: every task must receive a start slot.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .penalize(hard_weight(unary_weight))
        .named("complete_start")
}

fn unary_condition(entity: &Task) -> bool {
    entity.start_slot.is_none()
}

fn unary_weight(entity: &Task) -> HardSoftScore {
    if unary_condition(entity) {
        <HardSoftScore as Score>::one_hard()
    } else {
        <HardSoftScore as Score>::zero()
    }
}
