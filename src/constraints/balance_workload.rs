use crate::domain::{Plan, Task};
use solverforge::prelude::*;
use solverforge::stream::collector::LoadBalance;
use solverforge::IncrementalConstraint;

/// SOFT: balance assigned duration across discovered candidates.
pub fn constraint() -> impl IncrementalConstraint<Plan, HardSoftScore> {
    ConstraintFactory::<Plan, HardSoftScore>::new()
        .for_each(Plan::tasks())
        .group_by(
            balance_scope,
            load_balance(balance_group_key, balance_metric),
        )
        .penalize(balance_weight)
        .named("balance_workload")
}

fn balance_scope(_entity: &Task) -> usize {
    0
}

fn balance_group_key(entity: &Task) -> Option<usize> {
    entity.candidate_idx
}

fn balance_metric(entity: &Task) -> i64 {
    entity.duration_slots as i64
}

fn balance_weight(_scope: &usize, load: &LoadBalance<Option<usize>>) -> HardSoftScore {
    HardSoftScore::of_soft(load.unfairness())
}
