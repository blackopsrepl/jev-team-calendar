use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// SOFT: finish the task set as early as possible.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .penalize(unary_weight)
        .named("minimize_completion")
}

fn unary_weight(entity: &Task) -> HardSoftScore {
    HardSoftScore::of_soft(entity.end_slot().unwrap_or(0) as i64)
}
