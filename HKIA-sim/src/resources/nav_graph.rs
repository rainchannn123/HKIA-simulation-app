use bevy::prelude::*;
use crate::components::CheckpointId;

#[derive(Resource, Default)]
pub struct NavGraph {
    // TODO Phase 2: replace with petgraph::Graph<NavNode, NavEdge>
    // for full A* pathfinding across the airport layout.
}

pub struct NavNode {
    pub pos: Vec2,
    pub zone: ZoneType,
}

pub struct NavEdge {
    pub distance_m: f32,
    pub speed_limit_mps: f32,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ZoneType {
    Landside,
    SecurityZone,
    Airside,
    Apron,
}

impl NavGraph {
    /// Returns a straight-line path of waypoints to the given checkpoint.
    /// Phase 2 will replace this with real A* over the taxiway/corridor graph.
    pub fn path_to(&self, from: Vec2, destination: &CheckpointId) -> Vec<Vec2> {
        let target = checkpoint_position(destination);
        vec![from, target]
    }
}

/// Placeholder world positions for each checkpoint type.
/// Phase 2 will derive these from the loaded airport config.
fn checkpoint_position(id: &CheckpointId) -> Vec2 {
    match id {
        CheckpointId::Entry       => Vec2::new(-530.0, -280.0),
        CheckpointId::CheckIn     => Vec2::new(-530.0, -180.0),
        CheckpointId::Security    => Vec2::new(-530.0,  -80.0),
        CheckpointId::Immigration => Vec2::new(-530.0,   20.0),
        CheckpointId::Gate(id)    => match id.as_str() {
            "G1" => Vec2::new(-450.0, 120.0),
            "G2" => Vec2::new(-225.0, 120.0),
            "G3" => Vec2::new(   0.0, 120.0),
            "G4" => Vec2::new( 225.0, 120.0),
            "G5" => Vec2::new( 450.0, 120.0),
            _    => Vec2::ZERO,
        },
    }
}
