use bevy::prelude::*;
use crate::components::{BeingServed, CheckpointId, InQueue, Passenger, PathFollower};
use crate::resources::NavGraph;

// ── Plugin registration ───────────────────────────────────────────────────────

pub struct PassengerPlugin;

impl Plugin for PassengerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                move_passengers,
                process_queues,
                tick_service,
            )
                // enforce order: move → queue → serve, all in one frame
                .chain(),
        );
    }
}

// ── System 1: move passengers along their path ────────────────────────────────
// State: has PathFollower
// Exit:  remove PathFollower, add InQueue  (arrived at checkpoint)

fn move_passengers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut PathFollower, &Passenger)>,
) {
    for (entity, mut transform, mut follower, pax) in &mut query {
        // Guard: empty path should never happen, but skip safely if it does
        if follower.path.is_empty() {
            commands.entity(entity).remove::<PathFollower>();
            continue;
        }

        let target = follower.path[follower.current_node];
        let to_target = target - transform.translation.truncate();
        let distance = to_target.length();
        let step = follower.speed_mps * time.delta_secs();

        if step >= distance {
            // Snap to node and advance
            transform.translation = target.extend(transform.translation.z);
            follower.current_node += 1;

            if follower.current_node >= follower.path.len() {
                // Arrived at the checkpoint for this itinerary step
                if let Some(checkpoint) = pax.itinerary.get(pax.current_step) {
                    commands.entity(entity)
                        .remove::<PathFollower>()
                        .insert(InQueue { checkpoint: checkpoint.clone(), position: 0 });
                } else {
                    // Itinerary already finished — just remove the follower
                    commands.entity(entity).remove::<PathFollower>();
                }
            }
        } else {
            transform.translation += (to_target.normalize() * step).extend(0.0);
        }
    }
}

// ── System 2: admit passengers from queue into service ────────────────────────
// State: has InQueue
// Exit:  remove InQueue, add BeingServed  (reached the front and server is free)
//
// This uses a simple FIFO per checkpoint. Phase 4 will replace this with a
// proper M/M/c queue resource driven by the airport config (server count,
// mean service time).

fn process_queues(
    mut commands: Commands,
    mut queued: Query<(Entity, &InQueue, &mut Passenger)>,
) {
    // Group entities by checkpoint so we only admit one per checkpoint per frame.
    // (Replace with a proper server-pool in Phase 4.)
    use std::collections::HashSet;
    let mut checkpoints_served: HashSet<u8> = HashSet::new();

    // Sort by queue position so position 0 is always served first
    let mut entries: Vec<_> = queued.iter_mut().collect();
    entries.sort_by_key(|(_, q, _)| q.position);

    for (entity, queue, _pax) in entries {
        let key = checkpoint_key(&queue.checkpoint);
        if checkpoints_served.contains(&key) {
            continue; // server busy this frame
        }
        checkpoints_served.insert(key);

        let service_time = default_service_time(&queue.checkpoint);
        commands.entity(entity)
            .remove::<InQueue>()
            .insert(BeingServed {
                checkpoint: queue.checkpoint.clone(),
                time_remaining: service_time,
            });
    }
}

// ── System 3: tick service timer; advance itinerary when done ─────────────────
// State: has BeingServed
// Exit:  remove BeingServed, add PathFollower for next step
//        (or mark journey complete if itinerary is exhausted)

fn tick_service(
    mut commands: Commands,
    time: Res<Time>,
    nav_graph: Res<NavGraph>,
    mut query: Query<(Entity, &mut BeingServed, &mut Passenger, &Transform)>,
) {
    for (entity, mut service, mut pax, transform) in &mut query {
        service.time_remaining -= time.delta_secs();

        if service.time_remaining > 0.0 {
            continue;
        }

        // Service complete — advance to the next itinerary step
        pax.current_step += 1;
        commands.entity(entity).remove::<BeingServed>();

        if let Some(next_checkpoint) = pax.itinerary.get(pax.current_step) {
            let path = nav_graph.path_to(
                transform.translation.truncate(),
                next_checkpoint,
            );
            commands.entity(entity).insert(PathFollower {
                path,
                current_node: 0,
                speed_mps: 1.4, // average walking speed m/s
            });
        }
        // If no next step: passenger has completed their journey.
        // Phase 4 will fire a PassengerExitedEvent here for analytics.
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Unique u8 key per checkpoint variant for the HashSet in process_queues.
fn checkpoint_key(id: &CheckpointId) -> u8 {
    match id {
        CheckpointId::Entry       => 0,
        CheckpointId::CheckIn     => 1,
        CheckpointId::Security    => 2,
        CheckpointId::Immigration => 3,
        CheckpointId::Gate(_)     => 4, // all gates share one slot for now
    }
}

/// Default service time in seconds per checkpoint.
/// Phase 4 will load these from AirportConfig instead.
fn default_service_time(id: &CheckpointId) -> f32 {
    match id {
        CheckpointId::Entry       =>   5.0,
        CheckpointId::CheckIn     => 120.0,
        CheckpointId::Security    =>  45.0,
        CheckpointId::Immigration =>  60.0,
        CheckpointId::Gate(_)     =>  20.0,
    }
}
