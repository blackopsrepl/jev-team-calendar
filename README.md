# Jev Team Calendar

A SolverForge web application that discovers a project team from a shared directory of resumes.

```text
input/resumes/* + input/projects/<project>/tasks.json
                         |
                         v
              TypeSafe Jev preprocessing
                         |
                         v
generated/projects/<project>/jev_candidates.json
                         |
                         v
       SolverForge assignment and scheduling
```

Jev evaluates resume evidence only against the selected project's required skill universe. It does not assign tasks, choose times, score plans, select moves, or run during a solve. SolverForge receives thresholded capability facts and owns all combinatorial decisions.

## Run

```bash
solverforge server
```

Open <http://127.0.0.1:7860>. Select a project, use **Analyze resumes for this project** to rebuild its capability facts, then use **Solve**.

The application invokes `uv` itself. `uv` creates and maintains the ignored project-local `.venv`; users do not activate an environment or install the SDK manually. `TYPESAFE_API_KEY` is read from the process environment or the ignored local `.env` file.

Direct API operation:

```bash
curl -X POST http://127.0.0.1:7860/projects/platform-reliability/ingest
curl -X POST http://127.0.0.1:7860/projects/product-launch/ingest
```

`make ingest` remains a convenience for headless preprocessing of the default project, not the primary application workflow.

## Inputs

- `input/resumes/`: shared talent pool; `.md`, `.txt`, and `.pdf` are supported.
- `input/projects/platform-reliability/tasks.json`: infrastructure project and skill universe.
- `input/projects/product-launch/tasks.json`: product-launch project and distinct skill universe.
- `fixtures/projects/*/jev_candidates.json`: checked-in deterministic, zero-network test/demo artifacts.
- `generated/projects/*/jev_candidates.json`: live Jev output, intentionally ignored by Git.

PDF text extraction uses `pdftotext -layout`. Each resume produces one TypeSafe `system_one` call containing one independent Noul per distinct project skill. Noul's `.noul` value is the probability of “yes”; there is no separate Noul confidence field. `JEV_SKILL_THRESHOLD` defaults to `0.80`.

## SolverForge Model

- Fact: `Candidate` with resume source, project-qualified skills, raw probabilities in integer millionths, and explicit 90-slot availability.
- Entity: `Task` with required skills, duration, and permitted time window.
- Scalar variables: `Task.candidate_idx` and `Task.start_slot` (`0..90`).
- Score: `HardSoftScore`.

Hard constraints:

- complete candidate assignment;
- complete start-time assignment;
- every required skill present on the assigned candidate;
- no overlapping tasks for one candidate;
- task fits its time window and a single workday;
- every occupied slot is available for the candidate.

Soft constraints:

- earlier aggregate completion;
- fewer same-day idle gaps;
- balanced assigned duration.

## Verification

```bash
solverforge info
solverforge check
solverforge test
solverforge routes
uv run pytest
node --check static/app.js
```

The web shell retains the standard `/jobs` lifecycle and `solverforge-ui` controls, snapshots, SSE updates, score analysis, and raw Data view.

## Versions

- `solverforge-cli 3.1.0`
- `solverforge 0.19.5`
- `solverforge-ui 0.9.0`
- official `typesafe-sdk`, resolved in `uv.lock`
