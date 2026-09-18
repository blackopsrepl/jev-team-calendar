/* Constraint definitions.

Add constraint modules with `solverforge generate constraint ...`.
The neutral shell starts with an empty constraint set. */

use crate::domain::Plan;
use solverforge::prelude::*;

pub use self::assemble::create_constraints;

// @solverforge:begin constraint-modules
mod balance_workload;
mod candidate_availability;
mod complete_candidate;
mod complete_start;
mod minimize_completion;
mod minimize_idle_gaps;
mod no_candidate_overlap;
mod required_skills;
mod task_time_window;
// @solverforge:end constraint-modules

mod assemble {
    use super::*;

    pub fn create_constraints() -> impl ConstraintSet<Plan, HardSoftScore> {
        // @solverforge:begin constraint-calls
        (
            balance_workload::constraint(),
            candidate_availability::constraint(),
            complete_candidate::constraint(),
            complete_start::constraint(),
            minimize_completion::constraint(),
            minimize_idle_gaps::constraint(),
            no_candidate_overlap::constraint(),
            required_skills::constraint(),
            task_time_window::constraint(),
        )
        // @solverforge:end constraint-calls
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Candidate, Task};
    use solverforge::ConstraintSet;

    fn candidate(id: &str, skills: &[&str]) -> Candidate {
        Candidate::new(
            id,
            id,
            0,
            id.to_string(),
            skills.iter().map(|skill| (*skill).to_string()).collect(),
            skills.iter().map(|skill| (*skill).to_string()).collect(),
            vec![950_000; skills.len()],
            vec![true; 90],
            format!("input/resumes/{id}.txt"),
        )
    }

    fn task(id: &str, skills: &[&str], start: usize, duration: usize) -> Task {
        let mut task = Task::new(
            id,
            id.to_string(),
            String::new(),
            skills.iter().map(|skill| (*skill).to_string()).collect(),
            duration,
            0,
            90,
        );
        task.candidate_idx = Some(0);
        task.start_slot = Some(start);
        task
    }

    #[test]
    fn lacking_one_required_skill_is_rejected() {
        let plan = Plan::new(
            vec![candidate("alice", &["postgresql"])],
            vec![task(
                "migration",
                &["postgresql", "database_migration"],
                0,
                2,
            )],
        );
        assert_eq!(
            (required_skills::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_hard(-1)
        );
    }

    #[test]
    fn qualified_candidate_is_accepted() {
        let plan = Plan::new(
            vec![candidate("alice", &["postgresql", "database_migration"])],
            vec![task(
                "migration",
                &["postgresql", "database_migration"],
                0,
                2,
            )],
        );
        assert_eq!(
            (required_skills::constraint(),).evaluate_all(&plan),
            HardSoftScore::ZERO
        );
    }

    #[test]
    fn same_candidate_cannot_overlap() {
        let plan = Plan::new(
            vec![candidate("alice", &[])],
            vec![task("a", &[], 0, 4), task("b", &[], 2, 4)],
        );
        assert_eq!(
            (no_candidate_overlap::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_hard(-1)
        );
    }

    #[test]
    fn different_candidates_can_work_concurrently() {
        let mut second = task("b", &[], 0, 4);
        second.candidate_idx = Some(1);
        let plan = Plan::new(
            vec![candidate("alice", &[]), candidate("bob", &[])],
            vec![task("a", &[], 0, 4), second],
        );
        assert_eq!(
            (no_candidate_overlap::constraint(),).evaluate_all(&plan),
            HardSoftScore::ZERO
        );
    }

    #[test]
    fn same_candidate_overlap_is_detected_regardless_of_id_order() {
        let plan = Plan::new(
            vec![candidate("alice", &[])],
            vec![task("zulu", &[], 0, 4), task("alpha", &[], 2, 4)],
        );
        assert_eq!(
            (no_candidate_overlap::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_hard(-1)
        );
    }

    #[test]
    fn same_day_idle_gap_is_counted_regardless_of_id_order() {
        let plan = Plan::new(
            vec![candidate("alice", &[])],
            vec![task("zulu", &[], 0, 4), task("alpha", &[], 6, 4)],
        );
        assert_eq!(
            (minimize_idle_gaps::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_soft(-2)
        );
    }

    #[test]
    fn task_windows_are_enforced() {
        let mut outside = task("outside", &[], 2, 4);
        outside.earliest_start_slot = 4;
        outside.latest_end_slot = 12;
        let plan = Plan::new(vec![candidate("alice", &[])], vec![outside]);
        assert_eq!(
            (task_time_window::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_hard(-1)
        );
    }

    #[test]
    fn candidate_availability_is_enforced() {
        let mut unavailable = candidate("alice", &[]);
        unavailable.availability[2] = false;
        let plan = Plan::new(vec![unavailable], vec![task("a", &[], 1, 3)]);
        assert_eq!(
            (candidate_availability::constraint(),).evaluate_all(&plan),
            HardSoftScore::of_hard(-1)
        );
    }

    #[test]
    fn no_qualified_candidate_is_infeasible() {
        let plan = Plan::new(
            vec![candidate("alice", &["react"])],
            vec![task("migration", &["postgresql"], 0, 2)],
        );
        assert!(create_constraints().evaluate_all(&plan).hard() < 0);
    }
}
