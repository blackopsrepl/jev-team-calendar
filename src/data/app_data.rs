use std::{collections::BTreeMap, fs, path::Path, str::FromStr};

use chrono::{NaiveDate, NaiveDateTime};
use serde::Deserialize;

use crate::domain::{Candidate, Plan, Task};

const HORIZON_SLOTS: usize = 90;
const SLOTS_PER_DAY: usize = 18;
const SLOT_MINUTES: i64 = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DemoData {
    PlatformReliability,
    ProductLaunch,
}

const AVAILABLE: &[DemoData] = &[DemoData::PlatformReliability, DemoData::ProductLaunch];

pub fn default_demo_data() -> DemoData {
    DemoData::PlatformReliability
}

pub fn available_demo_data() -> &'static [DemoData] {
    AVAILABLE
}

impl DemoData {
    pub fn id(self) -> &'static str {
        match self {
            Self::PlatformReliability => "platform-reliability",
            Self::ProductLaunch => "product-launch",
        }
    }

    pub fn default_demo_data() -> Self {
        default_demo_data()
    }

    pub fn available_demo_data() -> &'static [Self] {
        available_demo_data()
    }
}

impl FromStr for DemoData {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        AVAILABLE
            .iter()
            .copied()
            .find(|project| project.id().eq_ignore_ascii_case(value))
            .ok_or(())
    }
}

#[derive(Deserialize)]
struct CandidateArtifact {
    candidates: Vec<InferredCandidate>,
}

#[derive(Deserialize)]
struct InferredCandidate {
    id: String,
    display_name: String,
    source: String,
    skills: BTreeMap<String, SkillJudgment>,
}

#[derive(Deserialize)]
struct SkillJudgment {
    qualified: bool,
    probability: f64,
}

#[derive(Deserialize)]
struct TaskInput {
    id: String,
    title: String,
    description: String,
    required_skills: Vec<RequiredSkill>,
    duration_minutes: usize,
    earliest_start: String,
    latest_end: String,
}

#[derive(Deserialize)]
struct RequiredSkill {
    id: String,
}

pub fn generate(project: DemoData) -> Plan {
    load_plan(project).expect("checked-in project and Jev fixture data must be valid")
}

fn load_plan(project: DemoData) -> Result<Plan, String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let generated = root.join(format!(
        "generated/projects/{}/jev_candidates.json",
        project.id()
    ));
    let candidate_path = if generated.exists() {
        generated
    } else {
        root.join(format!(
            "fixtures/projects/{}/jev_candidates.json",
            project.id()
        ))
    };
    let candidate_json = fs::read_to_string(&candidate_path)
        .map_err(|error| format!("read {}: {error}", candidate_path.display()))?;
    let artifact: CandidateArtifact = serde_json::from_str(&candidate_json)
        .map_err(|error| format!("parse {}: {error}", candidate_path.display()))?;

    let candidates = artifact
        .candidates
        .into_iter()
        .enumerate()
        .map(|(index, input)| {
            let evaluated_skills = input.skills.keys().cloned().collect::<Vec<_>>();
            let probabilities = input
                .skills
                .values()
                .map(|judgment| (judgment.probability.clamp(0.0, 1.0) * 1_000_000.0).round() as u32)
                .collect::<Vec<_>>();
            let qualified = input
                .skills
                .iter()
                .filter(|(_, judgment)| judgment.qualified)
                .map(|(skill, _)| skill.clone())
                .collect();
            Candidate::new(
                input.id.clone(),
                input.display_name.clone(),
                index,
                input.display_name,
                qualified,
                evaluated_skills,
                probabilities,
                vec![true; HORIZON_SLOTS],
                input.source,
            )
        })
        .collect();

    let task_path = root.join(format!("input/projects/{}/tasks.json", project.id()));
    let task_json = fs::read_to_string(&task_path)
        .map_err(|error| format!("read {}: {error}", task_path.display()))?;
    let inputs: Vec<TaskInput> = serde_json::from_str(&task_json)
        .map_err(|error| format!("parse {}: {error}", task_path.display()))?;
    let tasks = inputs
        .into_iter()
        .map(|input| {
            if input.duration_minutes == 0 || input.duration_minutes % 30 != 0 {
                return Err(format!(
                    "task {} duration must be a positive multiple of 30",
                    input.id
                ));
            }
            Ok(Task::new(
                input.id,
                input.title,
                input.description,
                input
                    .required_skills
                    .into_iter()
                    .map(|skill| skill.id)
                    .collect(),
                input.duration_minutes / 30,
                to_slot(&input.earliest_start)?,
                to_slot(&input.latest_end)?,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Plan::new(candidates, tasks))
}

fn to_slot(value: &str) -> Result<usize, String> {
    let date_time = NaiveDateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S")
        .map_err(|error| format!("invalid calendar timestamp {value}: {error}"))?;
    let horizon_start = NaiveDate::from_ymd_opt(2026, 9, 21)
        .unwrap()
        .and_hms_opt(9, 0, 0)
        .unwrap();
    let days = (date_time.date() - horizon_start.date()).num_days();
    let minute = i64::from(date_time.time().hour() * 60 + date_time.time().minute());
    let work_minute = minute - 9 * 60;
    if !(0..5).contains(&days)
        || !(0..=9 * 60).contains(&work_minute)
        || work_minute % SLOT_MINUTES != 0
    {
        return Err(format!(
            "timestamp {value} is outside the half-hour work-week grid"
        ));
    }
    let slot = days * SLOTS_PER_DAY as i64 + work_minute / SLOT_MINUTES;
    usize::try_from(slot).map_err(|_| format!("timestamp {value} precedes the horizon"))
}

use chrono::Timelike;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cached_jev_facts_and_tasks_deserialize() {
        let plan = load_plan(DemoData::PlatformReliability).unwrap();
        assert_eq!(plan.candidates.len(), 10);
        assert_eq!(plan.tasks.len(), 30);
        assert!(plan
            .candidates
            .iter()
            .all(|candidate| candidate.availability.len() == 90));
        assert!(plan.candidates.iter().any(|candidate| candidate
            .qualified_skills
            .contains(&"postgresql".to_string())));
    }

    #[test]
    fn projects_have_distinct_skill_universes() {
        let platform = load_plan(DemoData::PlatformReliability).unwrap();
        let launch = load_plan(DemoData::ProductLaunch).unwrap();
        assert!(platform
            .tasks
            .iter()
            .any(|task| task.required_skills.contains(&"postgresql".to_string())));
        assert!(launch
            .tasks
            .iter()
            .any(|task| task.required_skills.contains(&"ux_research".to_string())));
        assert_eq!(launch.candidates.len(), 10);
    }

    #[test]
    fn timestamps_map_to_work_week_slots() {
        assert_eq!(to_slot("2026-09-21T09:00:00").unwrap(), 0);
        assert_eq!(to_slot("2026-09-22T09:00:00").unwrap(), 18);
        assert_eq!(to_slot("2026-09-25T18:00:00").unwrap(), 90);
    }
}
