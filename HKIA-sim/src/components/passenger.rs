use bevy::prelude::*;

#[derive(Component)]
pub struct Passenger {
    pub id: u64,
    pub itinerary: Vec<CheckpointId>,  // ordered route
    pub current_step: usize,
}

/// The ordered stops a passenger travels through.
#[derive(Clone, PartialEq, Eq)]
pub enum CheckpointId {
    Entry,
    CheckIn,
    Security,
    Immigration,
    Gate(String),  // e.g. Gate("G3")
}

/// Attached while the passenger is walking between checkpoints.
#[derive(Component)]
pub struct PathFollower {
    pub path: Vec<Vec2>,   // waypoints from NavGraph A*
    pub current_node: usize,
    pub speed_mps: f32,
}

/// Attached while the passenger is waiting in a queue.
#[derive(Component)]
pub struct InQueue {
    pub checkpoint: CheckpointId,
    pub position: usize,  // their place in line
}

/// Attached while the passenger is actively being processed.
#[derive(Component)]
pub struct BeingServed {
    pub checkpoint: CheckpointId,
    pub time_remaining: f32,  // seconds until done
}