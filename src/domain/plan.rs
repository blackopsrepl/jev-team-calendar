use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

// @solverforge:neutral-solution
// @solverforge:begin solution-imports
use super::Candidate;
use super::Task;
// @solverforge:end solution-imports

/// The root planning solution.
///
/// Fresh projects start as a neutral shell. Add fact collections, planning
/// entity collections, and variable fields through the CLI as your domain
/// takes shape.
#[planning_solution(
    constraints = "crate::constraints::create_constraints",
    solver_toml = "../../solver.toml"
)]
#[derive(Serialize, Deserialize)]
pub struct Plan {
    // @solverforge:begin solution-collections
    #[problem_fact_collection]
    pub candidates: Vec<Candidate>,
    #[planning_entity_collection]
    pub tasks: Vec<Task>,
    // @solverforge:end solution-collections
    #[planning_score]
    pub score: Option<HardSoftScore>,
}

impl Plan {
    #[rustfmt::skip]
    pub fn new(
        // @solverforge:begin solution-constructor-params
        candidates: Vec<Candidate>,
        tasks: Vec<Task>,
        // @solverforge:end solution-constructor-params
    ) -> Self {
        let mut plan = Self {
            // @solverforge:begin solution-constructor-init
            candidates,
            tasks,
            // @solverforge:end solution-constructor-init
            score: None,
        };
        plan.normalize();
        plan
    }

    /// Restores dense fact indexes and range-safe planning values after transport.
    pub fn normalize(&mut self) {
        for (index, candidate) in self.candidates.iter_mut().enumerate() {
            candidate.index = index;
        }
        for task in &mut self.tasks {
            if task.candidate_idx.is_some_and(|index| index >= self.candidates.len()) {
                task.candidate_idx = None;
            }
            if task.start_slot.is_some_and(|slot| slot >= 90) {
                task.start_slot = None;
            }
        }
    }
}
