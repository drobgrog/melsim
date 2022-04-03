use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn teleportation_system(mut intersection_events: EventReader<IntersectionEvent>) {
    // For each teleporter ask - has the player collided with us?
    for intersection in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection);
    }
}
