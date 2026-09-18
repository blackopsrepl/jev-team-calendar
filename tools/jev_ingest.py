#!/usr/bin/env python3
"""Turn resume evidence into static candidate capability facts using Jev."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
from pathlib import Path
from typing import Any

from typesafe_sdk import Noul, TypeSafeClient


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_THRESHOLD = 0.80


def load_local_env(path: Path) -> None:
    if not path.exists():
        return
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line or line.lstrip().startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        os.environ.setdefault(key.strip(), value.strip().strip("'\""))


def extract_resume(path: Path) -> str:
    suffix = path.suffix.lower()
    if suffix in {".txt", ".md"}:
        return path.read_text(encoding="utf-8")
    if suffix == ".pdf":
        completed = subprocess.run(
            ["pdftotext", "-layout", str(path), "-"],
            check=True,
            capture_output=True,
            text=True,
        )
        return completed.stdout
    raise ValueError(f"unsupported resume format: {path}")


def load_skills(tasks_path: Path) -> dict[str, str]:
    tasks = json.loads(tasks_path.read_text(encoding="utf-8"))
    skills: dict[str, str] = {}
    for task in tasks:
        for skill in task["required_skills"]:
            existing = skills.setdefault(skill["id"], skill["description"])
            if existing != skill["description"]:
                raise ValueError(f"conflicting descriptions for skill {skill['id']}")
    return dict(sorted(skills.items()))


def skill_questions(skills: dict[str, str]) -> dict[str, Noul]:
    return {
        skill_id: Noul(
            instructions=(
                "Does `resume_text` provide sufficient evidence that the candidate "
                f"has this practical production skill: {description}?"
            ),
            criteria={
                "true": "The resume gives concrete professional evidence of this skill.",
                "false": "The skill is absent, merely adjacent, educational only, or too vague.",
            },
        )
        for skill_id, description in skills.items()
    }


def ingest_resume(
    client: TypeSafeClient,
    path: Path,
    skills: dict[str, str],
    threshold: float,
) -> dict[str, Any]:
    response = client.system_one(
        state={"resume_text": extract_resume(path)},
        questions=skill_questions(skills),
    )
    raw_http = response.raw_http_response.json()
    judgments = {
        skill_id: {
            "qualified": response.nouls[skill_id].noul >= threshold,
            "probability": response.nouls[skill_id].noul,
        }
        for skill_id in skills
    }
    return {
        "id": path.stem.lower().replace(" ", "-"),
        "display_name": path.stem.replace("_", " ").replace("-", " ").title(),
        "source": path.relative_to(ROOT).as_posix(),
        "skills": judgments,
        "raw": {
            "model": response.model,
            "request_id": response.request_id,
            "answers": raw_http.get("answers", {}),
            "usage": raw_http.get("usage", {}),
        },
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tasks", type=Path, default=ROOT / "input/projects/platform-reliability/tasks.json")
    parser.add_argument("--resumes", type=Path, default=ROOT / "input/resumes")
    parser.add_argument("--output", type=Path, default=ROOT / "generated/projects/platform-reliability/jev_candidates.json")
    parser.add_argument(
        "--threshold",
        type=float,
        default=float(os.environ.get("JEV_SKILL_THRESHOLD", DEFAULT_THRESHOLD)),
    )
    return parser.parse_args()


def main() -> None:
    load_local_env(ROOT / ".env")
    args = parse_args()
    if not 0.0 <= args.threshold <= 1.0:
        raise SystemExit("threshold must be between 0 and 1")
    if "TYPESAFE_API_KEY" not in os.environ:
        raise SystemExit("TYPESAFE_API_KEY is required (the local .env file is supported)")

    tasks_path = args.tasks if args.tasks.is_absolute() else ROOT / args.tasks
    resumes_path = args.resumes if args.resumes.is_absolute() else ROOT / args.resumes
    output_path = args.output if args.output.is_absolute() else ROOT / args.output
    skills = load_skills(tasks_path)
    resumes = sorted(
        path for path in resumes_path.iterdir() if path.suffix.lower() in {".txt", ".md", ".pdf"}
    )
    if not resumes:
        raise SystemExit(f"no .txt, .md, or .pdf resumes found in {resumes_path}")

    with TypeSafeClient() as client:
        candidates = [ingest_resume(client, path, skills, args.threshold) for path in resumes]

    artifact = {
        "schema_version": 1,
        "model": "jev-latest",
        "threshold": args.threshold,
        "candidates": candidates,
    }
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(artifact, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"wrote {len(candidates)} candidates and {len(skills)} skill judgments each to {output_path}")


if __name__ == "__main__":
    main()
