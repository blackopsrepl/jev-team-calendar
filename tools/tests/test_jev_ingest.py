from pathlib import Path

from tools.jev_ingest import extract_resume, load_skills, skill_questions


ROOT = Path(__file__).resolve().parents[2]


def test_task_skill_universe_is_deduplicated() -> None:
    skills = load_skills(ROOT / "input/projects/platform-reliability/tasks.json")
    assert len(skills) == 11
    assert "postgresql" in skills
    assert set(skill_questions(skills)) == set(skills)


def test_text_resume_extraction_is_local_and_deterministic() -> None:
    resume = ROOT / "input/resumes/alice.md"
    assert extract_resume(resume) == resume.read_text(encoding="utf-8")
