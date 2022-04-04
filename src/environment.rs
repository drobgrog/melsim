use crate::music::MusicState;
use crate::narrative::{NarrativeActions, NarrativeTextMessage};
use crate::{
    pickup::{spawn_pickup, Pickup},
    teleportation::add_teleporter,
};
use crate::{teleportation::Teleporter, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Location {
    Home,
    Park,
    Shops,
}

#[derive(Debug, Clone, Component)]
pub struct Environment {
    pub location: Location,
}

impl Environment {
    fn new(location: Location) -> Self {
        Self { location }
    }
}

pub fn setup_environment(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut music_state: ResMut<MusicState>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("environment.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(1000.0, 1000.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let x_pos = -(SCREEN_WIDTH / 2.) + 500.;
    let y_pos = ((SCREEN_HEIGHT / 2.) - 500.) - 30.;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: [x_pos, y_pos, 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Environment::new(Location::Home));

    create_environment(Location::Home, &mut commands, &mut music_state);

    // spawn_npc(&mut commands, &asset_server, [10, 12]);
    spawn_pickup(
        Pickup::Potplant,
        [10, 13],
        &mut commands,
        &asset_server,
        NarrativeActions::new_with_texts(vec![NarrativeTextMessage {
            sender: "Bowl of Petunias".into(),
            body: "Oh no, not again".into(),
        }]),
    );
}

pub fn create_environment(
    location: Location,
    commands: &mut Commands,
    music_state: &mut ResMut<MusicState>,
) {
    let (environment_colliders, mut teleporters) =
        get_environment_collider_and_teleporters(location);

    for collider in &environment_colliders {
        add_environment_collider(commands, collider);
    }

    for (environment_collider, teleporter) in teleporters.drain(..) {
        add_teleporter(commands, &environment_collider, teleporter);
    }

    let music_track_index = match location {
        Location::Home => 0,
        Location::Park => 1,
        Location::Shops => 2,
    };
    music_state.switch_tracks(music_track_index);
}

fn get_environment_collider_and_teleporters(
    location: Location,
) -> (
    Vec<EnvironmentCollider>,
    Vec<(EnvironmentCollider, Teleporter)>,
) {
    match location {
        Location::Home => {
            let environment_colliders = vec![
                EnvironmentCollider::new(0, 0, 11, 4),
                EnvironmentCollider::new(10, 0, 9, 1),
                EnvironmentCollider::new(15, 1, 4, 4),
                EnvironmentCollider::new(19, 1, 1, 19),
                EnvironmentCollider::new(0, 4, 3, 5),
                EnvironmentCollider::new(3, 4, 3, 1),
                EnvironmentCollider::new(6, 4, 1, 5),
                EnvironmentCollider::new(0, 8, 1, 11), // bottom left half of wall
                EnvironmentCollider::new(4, 16, 16, 4), // bottom area
            ];
            let teleporters = vec![(
                EnvironmentCollider::new(1, 19, 3, 1),
                Teleporter::new(Location::Park, [2, 3]),
            )];

            (environment_colliders, teleporters)
        }
        Location::Park => {
            let environment_colliders = vec![
                EnvironmentCollider::new(0, 0, 2, 20),
                EnvironmentCollider::new(2, 18, 18, 2),
                EnvironmentCollider::new(4, 0, 16, 2),
                EnvironmentCollider::new(18, 2, 2, 12),
                EnvironmentCollider::new(5, 2, 2, 2), // home sign
                EnvironmentCollider::new(2, 14, 3, 4), // tree
                EnvironmentCollider::new(14, 2, 4, 3), // swings
                EnvironmentCollider::new(16, 12, 2, 2), // shop sign
            ];
            let teleporters = vec![
                (
                    EnvironmentCollider::new(1, 1, 2, 1),
                    Teleporter::new(Location::Home, [2, 15]),
                ),
                (
                    EnvironmentCollider::new(18, 14, 2, 4),
                    Teleporter::new(Location::Shops, [2, 2]),
                ),
            ];

            (environment_colliders, teleporters)
        }
        Location::Shops => {
            let environment_colliders = vec![
                EnvironmentCollider::new(0, 0, 1, 20),   // far left wall
                EnvironmentCollider::new(1, 0, 19, 2),   // top wall
                EnvironmentCollider::new(1, 19, 19, 1),  // bottom wall
                EnvironmentCollider::new(18, 1, 20, 18), // far right wall
                EnvironmentCollider::new(6, 6, 10, 4),   // top isle
                EnvironmentCollider::new(4, 14, 10, 2),  // bottom isle
                EnvironmentCollider::new(1, 7, 1, 3),    // left of staffed checkout
                EnvironmentCollider::new(2, 8, 2, 2),    // right of staffed checkout
                EnvironmentCollider::new(1, 13, 3, 3),   // aut
            ];
            let teleporters = vec![
                (
                    EnvironmentCollider::new(0, 10, 1, 3),
                    Teleporter::new(Location::Park, [16, 14]),
                ),
                (
                    EnvironmentCollider::new(0, 16, 1, 3),
                    Teleporter::new(Location::Park, [16, 14]),
                ),
            ];

            (environment_colliders, teleporters)
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct EnvironmentCollider {
    pub x_coordinates: usize,
    pub y_coordinates: usize,
    pub width: usize,
    pub height: usize,
}

impl EnvironmentCollider {
    fn new(x_coordinates: usize, y_coordinates: usize, width: usize, height: usize) -> Self {
        Self {
            x_coordinates,
            y_coordinates,
            width,
            height,
        }
    }
}

fn add_environment_collider(commands: &mut Commands, environment_collider: &EnvironmentCollider) {
    let (x_pos, y_pos) = (
        environment_collider.x_coordinates,
        environment_collider.y_coordinates,
    );
    let (width, height) = (
        environment_collider.width as f32,
        environment_collider.height as f32,
    );

    let (collider_x, collider_y) = tile_coords_to_screen_pos(x_pos, width, y_pos, height);

    println!("COLLIDER: x pos: {:?}, y pos {:?}", collider_x, collider_y);
    commands
        .spawn_bundle(ColliderBundle {
            position: [collider_x / TILE_SIZE, collider_y / TILE_SIZE].into(),
            shape: ColliderShape::cuboid(width as f32 / 2., height as f32 / 2.).into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender::with_id(2))
        .insert(environment_collider.clone());
}

pub fn tile_coords_to_screen_pos(
    x_pos: usize,
    width: f32,
    y_pos: usize,
    height: f32,
) -> (f32, f32) {
    let collider_x = (-SCREEN_WIDTH / 2.) + (x_pos as f32 * TILE_SIZE) + ((width * TILE_SIZE) / 2.);
    let collider_y =
        (SCREEN_HEIGHT / 2.) - 30. - (y_pos as f32 * TILE_SIZE) - ((height * TILE_SIZE) / 2.);
    (collider_x, collider_y)
}
