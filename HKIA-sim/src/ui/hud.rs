use bevy::prelude::*;
use crate::components::SimLabel;
use crate::resources::SimulationClock;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_simulation);
    }
}

fn tick_simulation(
    time: Res<Time>,
    mut clock: ResMut<SimulationClock>,
    mut label: Query<&mut Text2d, With<SimLabel>>,
) {
    clock.elapsed += time.delta_secs_f64();

    for mut text in &mut label {
        text.0 = format!("Sim Time: {:.1}s", clock.elapsed);
    }
}
