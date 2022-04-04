use crate::music::MusicState;
use crate::{npc::spawn_npc, teleportation::Teleporter, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use crate::{
    pickup::{spawn_pickup, Pickup},
    teleportation::add_teleporter,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Location {
    Home,
    Park,
    Supermarket,
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

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform {
                translation: [-150., -30., 0.].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Environment::new(Location::Home));

    create_environment(Location::Home, &mut commands, &mut music_state);

    spawn_npc(&mut commands, &asset_server);
    spawn_pickup(Pickup::Potplant, [10, 13], &mut commands, &asset_server);
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
        Location::Supermarket => 2,
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
            ];
            let teleporters = vec![(
                EnvironmentCollider::new(1, 19, 3, 1),
                Teleporter::new(Location::Park, [2, 3]),
            )];

            (environment_colliders, teleporters)
        }
        Location::Park => {
            let environment_colliders = vec![EnvironmentCollider::new(0, 15, 11, 4)];
            let teleporters = vec![(
                EnvironmentCollider::new(1, 1, 3, 1),
                Teleporter::new(Location::Home, [2, 15]),
            )];

            (environment_colliders, teleporters)
        }
        Location::Supermarket => todo!(),
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

// TODO: These two functions are basically identical, split the logic out
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
