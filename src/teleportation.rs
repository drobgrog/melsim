use crate::environment::{create_environment, Environment, EnvironmentCollider};
use bevy::{ecs::system::Command, prelude::*};
use bevy_rapier2d::{na::Translation2, prelude::*};

use crate::{environment::Location, player::Player};

#[derive(Component, Debug, Clone)]
pub struct Teleporter {
    destination: Location,
    new_player_location: Translation2<f32>,
}

impl Teleporter {
    pub fn new(destination: Location, new_player_location: Translation2<f32>) -> Self {
        Self {
            destination,
            new_player_location,
        }
    }
}

pub fn teleportation_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    mut player_info: Query<(Entity, &mut RigidBodyPositionComponent), With<Player>>,
    teleporter_query: Query<&Teleporter>,
    mut environment_query: Query<(&mut TextureAtlasSprite, &mut Environment)>,
    environment_collider_query: Query<Entity, With<EnvironmentCollider>>,
) {
    let (player_entity, mut player_position) = player_info.single_mut();

    // For each teleporter ask - has the player collided with us?
    for (collider_a, collider_b, intersecting) in
        narrow_phase.intersections_with(player_entity.handle())
    {
        if intersecting {
            let teleporter_collider = if collider_a.entity() == player_entity {
                collider_b
            } else {
                collider_a
            };
            let teleporter = teleporter_query.get(teleporter_collider.entity()).unwrap();
            teleport(
                &teleporter,
                &mut player_position,
                &mut environment_query,
                &mut commands,
                &environment_collider_query,
            );
        }
    }
}

fn teleport(
    teleporter: &Teleporter,
    player_position: &mut Mut<RigidBodyPositionComponent>,
    environment_query: &mut Query<(&mut TextureAtlasSprite, &mut Environment)>,
    commands: &mut Commands,
    environment_collider_query: &Query<Entity, With<EnvironmentCollider>>,
) {
    let destination = teleporter.destination;
    // First, despawn the current environment
    for entity in environment_collider_query.iter() {
        commands.entity(entity).despawn();
    }

    // Then create the new environment
    create_environment(destination, commands);

    // Change the sprite
    let (mut sprite, mut environment) = environment_query.single_mut();
    sprite.index = match destination {
        Location::Home => 0,
        Location::Park => 1,
        Location::Supermarket => 2,
    };

    // Then move the player
    player_position.position.translation = teleporter.new_player_location;
    environment.location = destination;
}
