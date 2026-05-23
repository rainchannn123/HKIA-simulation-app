use bevy::prelude::*;

#[derive(Component)]
pub struct Aircraft {
    pub id: String,
    pub arrival_time: f64,
    pub departure_time: f64,
    pub gate: Option<String>,
}

#[derive(Component)]
pub struct Gate {
    pub id: String,
    pub capacity: usize,
}

/// Marker component for the HUD simulation clock text entity.
#[derive(Component)]
pub struct SimLabel;
