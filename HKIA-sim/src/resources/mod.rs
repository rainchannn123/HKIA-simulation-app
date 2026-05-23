pub mod clock;
pub mod nav_graph;

// Re-export so other modules can do:
// use crate::resources::SimulationClock;
// use crate::resources::NavGraph;
pub use clock::SimulationClock;
pub use nav_graph::NavGraph;
