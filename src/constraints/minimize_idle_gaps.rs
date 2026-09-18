use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::IncrementalConstraint;

/// SOFT: reduce same-day gaps between tasks assigned to one candidate.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .join(joiner::equal(candidate_idx_join_key))
        .penalize(pair_weight)
        .named("minimize_idle_gaps")
}

fn candidate_idx_join_key(entity: &Task) -> Option<usize> {
    entity.candidate_idx
}

fn pair_weight(left: &Task, right: &Task) -> HardSoftScore {
    if left.id >= right.id {
        return <HardSoftScore as Score>::zero();
    }
    let gap = match (
        left.start_slot,
        left.end_slot(),
        right.start_slot,
        right.end_slot(),
    ) {
        (Some(ls), Some(le), Some(rs), Some(re)) if ls / 18 == rs / 18 => {
            if le <= rs {
                rs - le
            } else if re <= ls {
                ls - re
            } else {
                0
            }
        }
        _ => 0,
    };
    HardSoftScore::of_soft(gap as i64)
}
