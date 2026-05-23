use bevy::prelude::*;
use crate::components::Aircraft;
use crate::resources::SimulationClock;

pub struct AircraftPlugin;

// Implementing plugin trait from bevy for our AircraftPlugin
impl Plugin for AircraftPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_events);
    }
}

fn handle_events(clock: Res<SimulationClock>, query: Query<&Aircraft>) {
    let t = clock.elapsed;
    for aircraft in &query {
        if (t - aircraft.arrival_time).abs() < 0.05 && t > 0.02 {
            println!(
                "[T={:.2}s] Aircraft {} arrived at gate {}",
                t,
                aircraft.id,
                aircraft.gate.as_deref().unwrap_or("unassigned")
            );
        }
        if (t - aircraft.departure_time).abs() < 0.05 {
            println!(
                "[T={:.2}s] Aircraft {} departed from gate {}",
                t,
                aircraft.id,
                aircraft.gate.as_deref().unwrap_or("unassigned")
            );
        }
    }
}
