use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SimulationClock {
    pub elapsed: f64,
}
