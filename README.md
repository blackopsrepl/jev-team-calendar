# Jev Team Calendar

<div align="center">

A shared resume pool goes in. A staffed, time-blocked project plan comes out.

[![Rust](https://img.shields.io/badge/rust-1.95%2B-orange.svg?logo=rust)](https://www.rust-lang.org)
[![SolverForge](https://img.shields.io/badge/solverforge-0.19.5-blue.svg)](https://crates.io/crates/solverforge)
[![solverforge-ui](https://img.shields.io/badge/solverforge--ui-0.9.0-blue.svg)](https://crates.io/crates/solverforge-ui)
[![TypeSafe Jev](https://img.shields.io/badge/TypeSafe-Jev-6f42c1.svg)](https://docs.typesafe.ai)

</div>

> **One boundary shapes the whole application.**
>
> Jev reads resumes and reports skill evidence. SolverForge makes every scheduling
> decision. The model never assigns a task, chooses a time, scores a plan, or
> selects a move, and it is never called from inside a solve.

A SolverForge web application that turns a directory of resumes into a qualified,
scheduled team for a chosen project. There is no employee roster and no
hand-maintained skill matrix: the people, their skills, and their availability all
come from the resumes.

Point it at a talent pool and a project task list. Jev evaluates each resume once,
before solving, and reports how likely each person is to have each of the
project's required skills. SolverForge then takes those thresholded capability
facts and works out who does what, and when.

```text
input/resumes/*  +  input/projects/<project>/tasks.json
                              |
                              v
                   TypeSafe Jev preprocessing
                   (one batched call per resume)
                              |
                              v
          generated/projects/<project>/jev_candidates.json
                              |
                              v
              SolverForge assignment + scheduling
                              |
                              v
                     team calendar + plan
```

## Quick Start

```bash
solverforge server
```

Open <http://127.0.0.1:7860>, choose a project, click **Analyze resumes for this
project**, then click **Solve**.

The application invokes `uv` itself. `uv` creates and maintains the ignored
project-local `.venv`, so there is no environment to activate and no SDK to
install by hand. `TYPESAFE_API_KEY` is read from the process environment or from
the ignored local `.env` file.

## How Team Discovery Works

1. **Project selection.** The chosen project defines the required skill universe
   and the work to be scheduled.
2. **Resume extraction.** `.md` and `.txt` resumes are read directly; `.pdf`
   resumes are converted with `pdftotext -layout`.
3. **Jev preprocessing.** Each resume produces one TypeSafe `system_one` request
   containing one independent `Noul` per distinct project skill. Noul's `.noul`
   value is the probability of *yes*; there is no separate confidence field.
4. **Thresholding.** A skill is `qualified` when its probability reaches
   `JEV_SKILL_THRESHOLD` (default `0.80`).
5. **Caching.** Results are written to the project's `jev_candidates.json`. The
   loader prefers the live generated artifact and falls back to the checked-in
   fixture.
6. **Solving.** SolverForge receives capability facts and the task list, and owns
   assignment and timing from there.

### Ingestion Is an Application Action

Ingestion is owned by the running application, not by a development chore. Trigger
it from the **Analyze resumes for this project** button in the UI, or directly:

```bash
curl -X POST http://127.0.0.1:7860/projects/platform-reliability/ingest
curl -X POST http://127.0.0.1:7860/projects/product-launch/ingest
```

The endpoint runs the preprocessing boundary in `tools/jev_ingest.py` and returns
when the candidate artifact has been rebuilt. Running the extractor by hand is not
part of the workflow.

## The SolverForge Model

| Role | Type | Notes |
|------|------|-------|
| Problem fact | `Candidate` | Resume source, project-qualified skills, raw probabilities in integer millionths, explicit 90-slot availability |
| Planning entity | `Task` | Required skills, duration, permitted time window |
| Scalar variable | `Task.candidate_idx` | `Option<usize>` over the discovered candidates |
| Scalar variable | `Task.start_slot` | `Option<usize>` over `0..90` |
| Score | `HardSoftScore` | Feasibility first, then three soft preferences |

The horizon is a five-day work week at 30-minute resolution: 18 slots per day,
90 slots total, Monday–Friday 09:00–18:00.

### Hard Constraints

| Constraint | Rule |
|------------|------|
| `complete_candidate` | Every task is assigned to a discovered candidate |
| `complete_start` | Every task receives a start slot |
| `required_skills` | The assignee holds every skill the task requires |
| `no_candidate_overlap` | One candidate cannot execute overlapping tasks |
| `task_time_window` | A task fits its allowed window and a single workday |
| `candidate_availability` | Every occupied slot is available for the assignee |

### Soft Constraints

| Constraint | Preference |
|------------|------------|
| `minimize_completion` | Finish the task set earlier |
| `minimize_idle_gaps` | Fewer same-day gaps between a candidate's tasks |
| `balance_workload` | Evenly distributed assigned duration |

A candidate who lacks a required skill is never silently substituted. The solve
simply reports the hard violation, which is why both demo projects solve to
`0hard` while an under-qualified pool would not.

## Project Data

- `input/resumes/` — the shared talent pool (10 resumes; `.md`, `.txt`, `.pdf`).
- `input/projects/platform-reliability/tasks.json` — 12 infrastructure tasks over
  11 skills.
- `input/projects/product-launch/tasks.json` — 10 launch tasks over a distinct
  9-skill universe.
- `fixtures/projects/*/jev_candidates.json` — deterministic, zero-network
  artifacts used by tests and as the fallback.
- `generated/projects/*/jev_candidates.json` — live Jev output, intentionally
  ignored by Git.

Each project's skill universe is derived from its own task list, so the same
resume can qualify for one project and not another.

## Repository Layout

```text
src/domain/        Candidate fact, Task entity, Plan solution
src/constraints/   Six hard and three soft constraints
src/data/          Project catalog and fixture/live artifact loading
src/api/           Ingest route, retained job lifecycle, DTOs, SSE, telemetry
tools/             TypeSafe Jev preprocessing boundary and Python tests
static/            Application UI on the shipped solverforge-ui shell
fixtures/          Checked-in deterministic candidate artifacts
input/             Resumes and per-project task definitions
```

## Web Interface

The shell is the standard `solverforge-ui` application, extended with:

- a **project selector** and an **Analyze resumes** action;
- a **discovered candidates** table showing qualified skills and resume source;
- a **project work** table of tasks and required skills;
- a **team calendar** timeline that renders each SolverForge assignment on the
  correct weekday and wall-clock time;
- the retained `/jobs` lifecycle, status, score analysis, snapshot, SSE, and raw
  Data views.

## HTTP API

| Method | Path | Purpose |
|--------|------|---------|
| `GET` | `/health` | Liveness |
| `GET` | `/info` | Application metadata |
| `GET` | `/demo-data` | Available projects |
| `GET` | `/demo-data/{id}` | Load a project's plan |
| `POST` | `/projects/{id}/ingest` | Rebuild that project's candidate facts |
| `POST` | `/jobs` | Start a solve |
| `GET` | `/jobs/{id}` | Retained job state |
| `GET` | `/jobs/{id}/snapshot` | Current best solution |
| `GET` | `/jobs/{id}/analysis` | Score analysis |
| `GET` | `/jobs/{id}/events` | Server-sent lifecycle events |
| `POST` | `/jobs/{id}/pause` \| `/resume` \| `/cancel` | Job control |

## Development

```bash
solverforge check      # managed model/constraint consistency
solverforge test       # Rust constraint and domain tests
solverforge routes     # HTTP route inventory
cargo check
uv run pytest          # zero-network preprocessing tests
node --check static/app.js
```

## Versions

- `solverforge-cli 3.1.0`
- `solverforge 0.19.5`
- `solverforge-ui 0.9.0`
- official `typesafe-sdk`, resolved in `uv.lock`
- Jev model reported by live ingestion: `jev-1.13.0`

## Status

**Current version:** 0.1.0

Both demo projects solve to `0hard` with distinct soft scores, and the generated
web application drives discovery, solving, and review end to end.
