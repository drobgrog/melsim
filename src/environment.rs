use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_environment(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("environment.png"),
        transform: Transform {
            translation: [-150., -30., 0.].into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let environment_colliders = vec![
        EnvironmentCollider::new(0, 0, 11, 4),
        EnvironmentCollider::new(10, 0, 9, 1),
        EnvironmentCollider::new(15, 1, 4, 4),
        EnvironmentCollider::new(19, 1, 1, 19),
        EnvironmentCollider::new(0, 4, 3, 5),
        EnvironmentCollider::new(3, 4, 3, 1),
        EnvironmentCollider::new(6, 4, 1, 5),
    ];
    for collider in &environment_colliders {
        add_collider(&mut commands, collider);
    }
}

struct EnvironmentCollider {
    x_coordinates: usize,
    y_coordinates: usize,
    width: usize,
    height: usize,
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

fn add_collider(commands: &mut Commands, collider: &EnvironmentCollider) {
    let (x_pos, y_pos) = (collider.x_coordinates, collider.y_coordinates);
    let (width, height) = (collider.width as f32, collider.height as f32);

    let collider_x = (-SCREEN_WIDTH / 2.) + (x_pos as f32 * TILE_SIZE) + ((width * TILE_SIZE) / 2.);
    let collider_y =
        (SCREEN_HEIGHT / 2.) - 30. - (y_pos as f32 * TILE_SIZE) - ((height * TILE_SIZE) / 2.);

    println!("COLLIDER: x pos: {:?}, y pos {:?}", collider_x, collider_y);
    commands
        .spawn_bundle(ColliderBundle {
            position: [collider_x / TILE_SIZE, collider_y / TILE_SIZE].into(),
            shape: ColliderShape::cuboid(width as f32 / 2., height as f32 / 2.).into(),
            ..Default::default()
        })
        .insert(ColliderDebugRender::with_id(2))
        .insert(ColliderPositionSync::Discrete);
}
