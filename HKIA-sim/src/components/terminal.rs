use bevy::prelude::*;

#[derive(Component)]
pub struct Terminal {
    pub id: String,
    pub gates: Vec<String>,
    pub passengers: Vec<u64>,
    pub aircraft: Vec<String>,
    pub auxiliary_structures: Vec<String>,
    pub position: Vec2,
    pub size: Vec2
}

#[derive(Component)]
pub struct AuxiliaryStructure {
    pub id: String,
    pub type_name: String,
    pub position: Vec2,
    pub size: Vec2
}

#[derive(Component)]
pub struct SecurityCheckpoint {
    pub id: String,
    pub length: f32,
    pub width: f32,
    pub position: Vec2,
    pub orientation: f32
}

#[derive(Component)]
pub struct ImmigrationCounter {
    pub id: String,
    pub position: Vec2,
    pub size: Vec2
}

#[derive(Component)]
pub struct Gate {
    pub id: String,
    pub position: Vec2,
    pub size: Vec2
}

#[derive(Component)]
pub struct BaggageClaim {
    pub id: u64,
    pub itinerary: Vec<String>, // sequence of checkpoints
}



