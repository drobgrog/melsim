use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{environment::Location, player::Player};

#[derive(Component, Debug, Clone)]
pub struct Teleporter {
    destination: Location,
}

impl Teleporter {
    pub fn new(destination: Location) -> Self {
        Self { destination }
    }
}

pub fn teleportation_system(
    narrow_phase: Res<NarrowPhase>,
    mut player_info: Query<(Entity, &mut RigidBodyPositionComponent), With<Player>>,
    teleporter_query: Query<&Teleporter>,
) {
    let (player_entity, mut player_position) = player_info.single_mut();

    // For each teleporter ask - has the player collided with us?
    for (teleporter_collider, _, intersecting) in
        narrow_phase.intersections_with(player_entity.handle())
    {
        if intersecting {
            let teleporter = teleporter_query.get(teleporter_collider.entity()).unwrap();
            println!("TELEPORTING TO {:?}", teleporter.destination);
            player_position.position.translation = [3.0, 5.0].into();
        }
    }
}
