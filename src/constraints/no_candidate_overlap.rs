use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// HARD: one candidate cannot execute overlapping tasks.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .join(joiner::equal(candidate_idx_join_key))
        .penalize(hard_weight(pair_weight))
        .named("no_candidate_overlap")
}

fn candidate_idx_join_key(entity: &Task) -> Option<usize> {
    entity.candidate_idx
}

fn pair_condition(left: &Task, right: &Task) -> bool {
    if left.id >= right.id {
        return false;
    }
    match (
        left.start_slot,
        left.end_slot(),
        right.start_slot,
        right.end_slot(),
    ) {
        (Some(left_start), Some(left_end), Some(right_start), Some(right_end)) => {
            left_start < right_end && right_start < left_end
        }
        _ => false,
    }
}

fn pair_weight(left: &Task, right: &Task) -> HardSoftScore {
    if pair_condition(left, right) {
        <HardSoftScore as Score>::one_hard()
    } else {
        <HardSoftScore as Score>::zero()
    }
}
