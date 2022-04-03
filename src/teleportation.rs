use crate::{
    environment::{
        create_environment, tile_coords_to_screen_pos, Environment, EnvironmentCollider,
    },
    TILE_SIZE,
};
use bevy::prelude::*;
use bevy_rapier2d::{na::Translation2, prelude::*};

use crate::{environment::Location, player::Player};

#[derive(Component, Debug, Clone)]
pub struct Teleporter {
    destination: Location,
    new_player_location: Translation2<f32>,
}

impl Teleporter {
    pub fn new(destination: Location, player: [usize; 2]) -> Self {
        let (x, y) = tile_coords_to_screen_pos(player[0], 2., player[1], 3.);
        Self {
            destination,
            new_player_location: [x / TILE_SIZE, y / TILE_SIZE].into(),
        }
    }
}

pub fn teleportation_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    mut player_info: Query<&mut RigidBodyPositionComponent, With<Player>>,
    teleporter_query: Query<(Entity, &Teleporter)>,
    mut environment_query: Query<(&mut TextureAtlasSprite, &mut Environment)>,
    environment_collider_query: Query<Entity, With<EnvironmentCollider>>,
) {
    let mut player_position = player_info.single_mut();

    // For each teleporter ask - has the player collided with us?
    for (teleporter_entity, teleporter) in teleporter_query.iter() {
        for (_, _, intersecting) in narrow_phase.intersections_with(teleporter_entity.handle()) {
            if intersecting {
                teleport(
                    teleporter,
                    &mut player_position,
                    &mut environment_query,
                    &mut commands,
                    &environment_collider_query,
                );
            }
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
    println!("Moving player to {:?}", teleporter.new_player_location);
    environment.location = destination;
}

pub fn add_teleporter(
    commands: &mut Commands,
    environment_collider: &EnvironmentCollider,
    teleporter: Teleporter,
) {
    let (x_pos, y_pos) = (
        environment_collider.x_coordinates,
        environment_collider.y_coordinates,
    );
    let (width, height) = (
        environment_collider.width as f32,
        environment_collider.height as f32,
    );

    let (collider_x, collider_y) = tile_coords_to_screen_pos(x_pos, width, y_pos, height);

    let collider_flags = ColliderFlags {
        active_events: ActiveEvents::all(),
        ..Default::default()
    }
    .into();

    println!(
        "TELEPORTER: x pos: {:?}, y pos {:?}",
        collider_x, collider_y
    );
    commands
        .spawn_bundle(ColliderBundle {
            flags: collider_flags,
            collider_type: ColliderType::Sensor.into(),
            position: [collider_x / TILE_SIZE, collider_y / TILE_SIZE].into(),
            shape: ColliderShape::cuboid(width as f32 / 2., height as f32 / 2.).into(),
            ..Default::default()
        })
        .insert(teleporter)
        .insert(environment_collider.clone());
}
