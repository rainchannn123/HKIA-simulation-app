// This file has been split into the plugin system.
// setup          → src/plugins/terminal.rs  (TerminalPlugin)
// tick_simulation → src/ui/hud.rs           (HudPlugin)
// handle_events  → src/plugins/aircraft.rs  (AircraftPlugin)


pub fn setup(mut commands: Commands) {
    // 2D camera
    commands.spawn(Camera2d);

    // Runway strip
    commands.spawn((
        Sprite {
            color: Color::srgb(0.18, 0.18, 0.20),
            custom_size: Some(Vec2::new(1150.0, 80.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -120.0, 0.0),
    ));

    // Gates
    let gates = [
        ("G1", -450.0_f32),
        ("G2", -225.0),
        ("G3", 0.0),
        ("G4", 225.0),
        ("G5", 450.0),
    ];

    for (id, x) in gates {
        // Gate body
        commands.spawn((
            Gate { id: id.to_string(), capacity: 1 },
            Sprite {
                color: Color::srgb(0.22, 0.42, 0.72),
                custom_size: Some(Vec2::new(90.0, 70.0)),
                ..default()
            },
            Transform::from_xyz(x, 120.0, 0.0),
        ));

        // Gate label
        commands.spawn((
            Text2d::new(id),
            TextFont { font_size: 16.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(x, 120.0, 1.0),
        ));

        // Jetway connector
        commands.spawn((
            Sprite {
                color: Color::srgb(0.35, 0.35, 0.38),
                custom_size: Some(Vec2::new(10.0, 40.0)),
                ..default()
            },
            Transform::from_xyz(x, 65.0, 0.0),
        ));
    }

    // Aircraft parked at gates
    let aircraft_data = [
        ("A123", 0.0_f64, 90.0_f64, "G1", -450.0_f32),
        ("B456", 5.0, 120.0, "G3", 0.0),
        ("C789", 10.0, 150.0, "G5", 450.0),
    ];

    for (id, arrival, departure, gate, x) in aircraft_data {
        // Aircraft body
        commands.spawn((
            Aircraft {
                id: id.to_string(),
                arrival_time: arrival,
                departure_time: departure,
                gate: Some(gate.to_string()),
            },
            Sprite {
                color: Color::srgb(0.88, 0.74, 0.20),
                custom_size: Some(Vec2::new(64.0, 28.0)),
                ..default()
            },
            Transform::from_xyz(x, 30.0, 1.0),
        ));

        // Aircraft ID label
        commands.spawn((
            Text2d::new(id),
            TextFont { font_size: 13.0, ..default() },
            TextColor(Color::srgb(0.95, 0.95, 0.95)),
            Transform::from_xyz(x, 14.0, 2.0),
        ));
    }

    // Sample passengers
    for i in 0..10_u64 {
        commands.spawn(Passenger {
            id: i,
            itinerary: vec![
                "Check-in".to_string(),
                "Security".to_string(),
                format!("G{}", (i % 5) + 1),
            ],
        });
    }

    // HUD — simulation clock
    commands.spawn((
        SimLabel,
        Text2d::new("Sim Time: 0.0s"),
        TextFont { font_size: 22.0, ..default() },
        TextColor(Color::srgb(0.85, 0.90, 0.95)),
        Transform::from_xyz(-530.0, 330.0, 2.0),
    ));
}

pub fn tick_simulation(
    time: Res<Time>,
    mut clock: ResMut<SimulationClock>,
    mut label: Query<&mut Text2d, With<SimLabel>>,
) {
    clock.elapsed += time.delta_secs_f64();

    for mut text in &mut label {
        text.0 = format!("Sim Time: {:.1}s", clock.elapsed);
    }
}

pub fn handle_events(clock: Res<SimulationClock>, query: Query<&Aircraft>) {
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