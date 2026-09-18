use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: tasks must fit their window and one workday.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .penalize(hard_weight(unary_weight))
        .named("task_time_window")
}

fn unary_condition(entity: &Task) -> bool {
    let Some(start) = entity.start_slot else {
        return false;
    };
    let Some(end) = entity.end_slot() else {
        return true;
    };
    start < entity.earliest_start_slot
        || end > entity.latest_end_slot
        || start / 18 != end.saturating_sub(1) / 18
}

fn unary_weight(entity: &Task) -> HardSoftScore {
    if unary_condition(entity) {
        <HardSoftScore as Score>::one_hard()
    } else {
        <HardSoftScore as Score>::zero()
    }
}
