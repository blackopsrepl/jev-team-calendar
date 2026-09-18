use crate::domain::{Candidate, Plan, Task};
use solverforge::prelude::*;
use solverforge::stream::joiner::equal_bi;
use solverforge::IncrementalConstraint;

/// HARD: every occupied slot must be available for the assignee.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .join((Plan::candidates(), equal_bi(entity_join_key, fact_join_key)))
        .penalize(hard_weight(join_weight))
        .named("candidate_availability")
}

fn entity_join_key(entity: &Task) -> Option<usize> {
    entity.candidate_idx
}

fn fact_join_key(fact: &Candidate) -> Option<usize> {
    Some(fact.index)
}

fn join_condition(entity: &Task, fact: &Candidate) -> bool {
    let Some(start) = entity.start_slot else {
        return false;
    };
    let Some(end) = entity.end_slot() else {
        return true;
    };
    (start..end).any(|slot| !fact.availability.get(slot).copied().unwrap_or(false))
}

fn join_weight(entity: &Task, fact: &Candidate) -> HardSoftScore {
    if join_condition(entity, fact) {
        <HardSoftScore as Score>::one_hard()
    } else {
        <HardSoftScore as Score>::zero()
    }
}
