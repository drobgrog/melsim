use crate::{
    environment::{
        create_environment, tile_coords_to_screen_pos, Environment, EnvironmentCollider,
    },
    music::MusicState,
    npc::{spawn_npc, NPC},
    pickup::Pickup,
    sfx::{SFXSystem, SoundEffect},
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
    mut player_info: Query<(Entity, &mut RigidBodyPositionComponent), With<Player>>,
    teleporter_query: Query<(Entity, &Teleporter)>,
    mut environment_query: Query<(&mut TextureAtlasSprite, &mut Environment)>,
    environment_collider_query: Query<Entity, With<EnvironmentCollider>>,
    mut music_state: ResMut<MusicState>,
    asset_server: Res<AssetServer>,
    mut sfx_system: ResMut<SFXSystem>,
    npc_query: Query<(Entity, &NPC)>,
    pickup_query: Query<(Entity, &Pickup)>,
) {
    let (player_entity, mut player_position) = player_info.single_mut();

    // For each teleporter ask - has the player collided with us?
    for (teleporter_entity, teleporter) in teleporter_query.iter() {
        for (collider_a, collider_b, intersecting) in
            narrow_phase.intersections_with(teleporter_entity.handle())
        {
            if collider_a.entity() == player_entity || collider_b.entity() == player_entity {
                sfx_system.play_sfx(SoundEffect::EntranceExit);
                if intersecting {
                    for (entity, _) in npc_query.iter() {
                        commands.entity(entity).despawn();
                    }
                    for (entity, _) in pickup_query.iter() {
                        commands.entity(entity).despawn();
                    }

                    if teleporter.destination == Location::Park {
                        spawn_npc(&mut commands, &asset_server, [5, 14]);
                    }
                    if teleporter.destination == Location::Shops {
                        spawn_npc(&mut commands, &asset_server, [6, 14]);
                    }

                    teleport(
                        teleporter,
                        &mut player_position,
                        &mut environment_query,
                        &mut commands,
                        &environment_collider_query,
                        &mut music_state,
                        &asset_server,
                    );
                }
            }
        }
    }
}

pub fn teleport(
    teleporter: &Teleporter,
    player_position: &mut Mut<RigidBodyPositionComponent>,
    environment_query: &mut Query<(&mut TextureAtlasSprite, &mut Environment)>,
    commands: &mut Commands,
    environment_collider_query: &Query<Entity, With<EnvironmentCollider>>,
    music_state: &mut ResMut<MusicState>,
    asset_server: &Res<AssetServer>,
) {
    let destination = teleporter.destination;
    // First, despawn the current environment
    for entity in environment_collider_query.iter() {
        commands.entity(entity).despawn();
    }

    // Then create the new environment
    create_environment(destination, commands, music_state);
    match destination {
        Location::Park => {
            // spawn_npc(commands, asset_server);
        }
        _ => {}
    };

    // Change the sprite
    let (mut sprite, mut environment) = environment_query.single_mut();
    sprite.index = match destination {
        Location::Home => 0,
        Location::Park => 1,
        Location::Shops => 2,
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
