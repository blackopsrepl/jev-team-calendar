use serde::{Deserialize, Serialize};
use solverforge::prelude::*;

/// A person discovered from a resume and classified by Jev before solving.
#[problem_fact]
#[derive(Serialize, Deserialize)]
pub struct Candidate {
    #[planning_id]
    pub id: String,
    pub name: String,
    pub index: usize,
    pub display_name: String,
    pub qualified_skills: Vec<String>,
    /// Skill probabilities in millionths, aligned with `evaluated_skills`.
    pub evaluated_skills: Vec<String>,
    pub skill_probabilities_ppm: Vec<u32>,
    pub availability: Vec<bool>,
    pub resume_source: String,
}

impl Candidate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(id: impl Into<String>, name: impl Into<String>, index: usize, display_name: String, qualified_skills: Vec<String>, evaluated_skills: Vec<String>, skill_probabilities_ppm: Vec<u32>, availability: Vec<bool>, resume_source: String) -> Self {
        Self { id: id.into(), name: name.into(), index, display_name, qualified_skills, evaluated_skills, skill_probabilities_ppm, availability, resume_source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_construction() {
        let fact = Candidate::new("test-id", "test", Default::default(), "test".to_string(), Default::default(), Default::default(), Default::default(), Default::default(), "test".to_string());
        assert_eq!(fact.id, "test-id");
        assert_eq!(fact.name, "test");
        let _ = &fact.index;
        let _ = &fact.display_name;
        let _ = &fact.qualified_skills;
        let _ = &fact.evaluated_skills;
        let _ = &fact.skill_probabilities_ppm;
        let _ = &fact.availability;
        let _ = &fact.resume_source;
    }
}
