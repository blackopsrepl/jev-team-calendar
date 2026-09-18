solverforge::planning_model! {
    root = "src/domain";

    // @solverforge:begin domain-exports
mod candidate;
mod task;
mod plan;

pub use candidate::Candidate;
pub use task::Task;
pub use plan::Plan;
    // @solverforge:end domain-exports
}
