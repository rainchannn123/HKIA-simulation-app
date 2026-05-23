pub mod aircraft;
pub mod navigation;
pub mod passenger;
pub mod vehicle;

// Re-export all components so other modules can do:
// use crate::components::{Aircraft, Gate, Passenger, SimLabel};
pub use aircraft::{Aircraft, Gate, SimLabel};
pub use passenger::{BeingServed, CheckpointId, InQueue, Passenger, PathFollower};
