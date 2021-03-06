use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    environment::tile_coords_to_screen_pos,
    game::GameState,
    narrative::NarrativeActions,
    player::Player,
    sfx::{SFXSystem, SoundEffect},
    TILE_SIZE,
};

#[derive(Component, Debug, Clone)]
pub enum Pickup {
    Potplant,
}

pub fn pickup_system(
    mut commands: Commands,
    narrow_phase: Res<NarrowPhase>,
    pickup_query: Query<(Entity, &Pickup, &NarrativeActions)>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
    player_query: Query<(Entity, &Player, &Transform)>,
    mut sfx_system: ResMut<SFXSystem>,
) {
    // For each pickup - ask did someone collide with us?
    for (pickup_entity, pickup, narrative_actions) in pickup_query.iter() {
        for (collider_a, collider_b, intersecting) in
            narrow_phase.intersections_with(pickup_entity.handle())
        {
            let collector = if collider_a.entity() == pickup_entity {
                collider_b
            } else {
                collider_a
            };
            if intersecting {
                collect_pickup(pickup, pickup_entity, collector.entity(), &mut commands);
                let (player_entity, _, player_transform) = player_query.single();

                if collector.entity() == player_entity {
                    sfx_system.play_sfx(SoundEffect::Pickup);

                    game_state.do_narrative_actions(
                        narrative_actions.clone(),
                        &time,
                        &mut commands,
                        &asset_server,
                        player_transform,
                        &mut sfx_system,
                    );
                }
            }
        }
    }
}

pub fn spawn_pickup(
    pickup: Pickup,
    location: [usize; 2],
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    narrative_actions: NarrativeActions,
) {
    let texture = get_image(&pickup, asset_server);
    let (width, height) = get_dimensions(&pickup);
    let (x_pos, y_pos) = (location[0], location[1]);
    let (collider_x, collider_y) = tile_coords_to_screen_pos(x_pos, width, y_pos, height);

    let collider_flags = ColliderFlags {
        active_events: ActiveEvents::all(),
        ..Default::default()
    }
    .into();

    commands
        .spawn_bundle(SpriteBundle {
            texture,
            transform: Transform {
                translation: [collider_x, collider_y, 1.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            flags: collider_flags,
            collider_type: ColliderType::Sensor.into(),
            position: [collider_x / TILE_SIZE, collider_y / TILE_SIZE].into(),
            shape: ColliderShape::cuboid(width as f32 / 2., height as f32 / 2.).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(narrative_actions)
        .insert(pickup);
}

fn get_dimensions(pickup: &Pickup) -> (f32, f32) {
    match pickup {
        Pickup::Potplant => (3., 3.),
    }
}

fn get_image(pickup: &Pickup, asset_server: &AssetServer) -> Handle<Image> {
    let path = match pickup {
        Pickup::Potplant => "potplant.png",
    };
    asset_server.load(path)
}

fn collect_pickup(
    pickup: &Pickup,
    pickup_entity: Entity,
    collector: Entity,
    commands: &mut Commands,
) {
    println!("{:?} picked up {:?}", collector, pickup);
    commands.entity(pickup_entity).despawn();
}
