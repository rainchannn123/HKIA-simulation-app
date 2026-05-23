use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod ui;
mod world;

use resources::{NavGraph, SimulationClock};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HKIA Airport Simulation".into(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.08, 0.12)))
        .insert_resource(SimulationClock::default())
        .insert_resource(NavGraph::default())
        .add_plugins((
            plugins::terminal::TerminalPlugin,
            plugins::aircraft::AircraftPlugin,
            plugins::passenger::PassengerPlugin,
            ui::hud::HudPlugin,
        ))
        .run();
}
