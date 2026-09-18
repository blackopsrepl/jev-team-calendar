# jev-team-calendar

A SolverForge constraint optimization project (scaffold: `neutral web scaffold`).

## Versioning

- CLI version used to scaffold this project: `3.1.0`
- SolverForge runtime target for this scaffold: `solverforge 0.19.5`
- SolverForge UI target for this scaffold: `solverforge-ui 0.9.0`
- SolverForge maps target for this scaffold: `solverforge-maps 2.1.4`
- Runtime dependency currently wired into `Cargo.toml`: `crates.io: solverforge 0.19.5`
- Frontend UI dependency currently wired into `Cargo.toml`: `crates.io: solverforge-ui 0.9.0`
- Maps dependency currently wired into `Cargo.toml`: `crates.io: solverforge-maps 2.1.4`
- Scaffold shell: `web`

This project was scaffolded by `solverforge-cli`, and it currently targets `SolverForge crate target 0.19.5` through the configured crate dependency targets.

## Quick Start

```bash
# Start the solver server
solverforge server

# Or run directly
cargo run --release
```

## Development

```bash
# Add a new constraint
solverforge generate constraint my_rule --unary --hard

# Add a problem fact
solverforge generate fact resource --field category:String --field load:i32

# Add a domain entity
solverforge generate entity task --field label:String --field priority:i32

# Add a scalar planning variable
solverforge generate variable resource_idx --entity Task --kind scalar --range resources --allows-unassigned

# Or add a scalar over a non-negative half-open integer range
# solverforge generate variable hour --entity Task --kind scalar --countable-range 0..24

# Or add scalar hook metadata when your domain owns the hook functions
# solverforge generate variable resource_idx --entity Task --kind scalar --range resources --candidate-values resource_candidates

# Add an ordered list variable with optional current SolverForge metadata
# solverforge generate variable visit_order --entity Route --kind list --elements visits --domain cvrp

# Enable bounded candidate-pull diagnostics when needed
# solverforge config set candidate_trace.max_entries 100000

# Remove a resource
solverforge destroy constraint my_rule
```

## Project Structure

| Directory | Purpose |
|-----------|--------|
| `src/domain/` | Planning entities, facts, and solution struct |
| `src/constraints/` | Constraint definitions (scored by the solver) |
| `src/solver/` | Solver service and configuration |
| `src/api/` | HTTP routes and DTOs |
| `src/data/` | Data loading and generation |
| `solverforge.app.toml` | Scaffolded app/domain contract |
| `solver.toml` | Solver configuration (termination, phases) |

## Runtime Diagnostics

Status, snapshot, and SSE payloads expose the complete compact SolverForge telemetry surface. Candidate pulls remain separate from ordinary control-plane traffic. After enabling `candidate_trace.max_entries`, use `GET /jobs/{id}/telemetry` for the atomically retained bounded trace and `POST /jobs/qualified` for externally attested qualified trace jobs.
