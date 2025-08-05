use bevy::prelude::*;
use bevy_butler::*;

use crate::utilities::UtilitiesPlugin;

// Why two events?
// The first event is public and is expected to be observed with commands.trigger
// The second event is private and is used to queue up a bunch of despawns
// It's processed at the end of the frame so despawns only happen then which avoids
// issues with despawning entities in observers and breaking things

/// Used to trigger a delayed despawn of an entity. Despawn happens at the end of the frame
#[derive(Event)]
pub struct DelayedDespawnEvent;

#[derive(Event)]
#[add_event(plugin = UtilitiesPlugin)]
struct DelayedBulkDespawnEvent(Entity);

#[add_observer(plugin = UtilitiesPlugin)]
fn watch_delayed_despawn(
    trigger: Trigger<DelayedDespawnEvent>,
    mut event_writer: EventWriter<DelayedBulkDespawnEvent>,
) {
    event_writer.send(DelayedBulkDespawnEvent(trigger.target()));
}

#[add_system(plugin = UtilitiesPlugin, schedule = Last)]
fn destroy_system(mut commands: Commands, mut event_reader: EventReader<DelayedBulkDespawnEvent>) {
    for event in event_reader.read() {
        commands.entity(event.0).despawn();
    }
}
