use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

/// Work whose assignee and start slot are chosen entirely by SolverForge.
#[planning_entity]
#[derive(Serialize, Deserialize)]
pub struct Task {
    #[planning_id]
    pub id: String,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<String>,
    pub duration_slots: usize,
    pub earliest_start_slot: usize,
    pub latest_end_slot: usize,
    // @solverforge:begin entity-variables
    #[planning_variable(value_range_provider = "candidates", allows_unassigned = true)]
    pub candidate_idx: Option<usize>,
    #[planning_variable(countable_range = "0..90", allows_unassigned = true)]
    pub start_slot: Option<usize>,
    // @solverforge:end entity-variables
}

impl Task {
    pub fn new(id: impl Into<String>, title: String, description: String, required_skills: Vec<String>, duration_slots: usize, earliest_start_slot: usize, latest_end_slot: usize) -> Self {
        Self {
            id: id.into(),
            title,
            description,
            required_skills,
            duration_slots,
            earliest_start_slot,
            latest_end_slot,
            // @solverforge:begin entity-variable-init
            candidate_idx: None,
            start_slot: None,
            // @solverforge:end entity-variable-init
        }
    }

    pub fn end_slot(&self) -> Option<usize> {
        self.start_slot
            .and_then(|start| start.checked_add(self.duration_slots))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_construction() {
        let entity = Task::new("test-id", "test".to_string(), "test".to_string(), Default::default(), Default::default(), Default::default(), Default::default());
        assert_eq!(entity.id, "test-id");
        let _ = &entity.title;
        let _ = &entity.description;
        let _ = &entity.required_skills;
        let _ = &entity.duration_slots;
        let _ = &entity.earliest_start_slot;
        let _ = &entity.latest_end_slot;
    }
}
