//! SWARMS runtime library — deterministic, self-contained Rust coordinator.

pub mod adapter;
pub mod cli;
pub mod config;
pub mod model;
pub mod quota;
pub mod resources;
pub mod review;
pub mod runtime;
pub mod session;
pub mod steering;
pub mod telemetry;
pub mod workflow_ir;

pub use model::{slug, Task, TaskSpec};

#[cfg(test)]
mod tests;
