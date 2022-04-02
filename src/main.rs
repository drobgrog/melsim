mod game;
mod player_physics;
mod ui;
use bevy::prelude::*;
use bevy_rapier2d::{na::Isometry2, prelude::*};
use player_physics::{player_movement, Player};

const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_WIDTH: f32 = 1300.0;
const TILE_SIZE: f32 = 50.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .init_resource::<game::GameState>()
        .add_startup_system(ui::setup_ui)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(ui::update)
        .add_system(game::logic)
        .add_plugin(RapierRenderPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    println!("Window scale: {:?}", window.scale_factor());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let sprite_size_x = 100.0;
    let sprite_size_y = 150.0;

    println!("Sprite size: x: {:?} y: {:?}", sprite_size_x, sprite_size_y);

    // Set the scale
    rapier_config.scale = TILE_SIZE;

    // Set gravity
    rapier_config.gravity = [0., 0.].into();

    // Set the size of the collider
    let collider_size_x = (sprite_size_x / rapier_config.scale) / 2.;
    let collider_size_y = (sprite_size_y / rapier_config.scale) / 2.;

    println!(
        "Collider size: x: {:?} y: {:?}",
        collider_size_x, collider_size_y
    );

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         texture: asset_server.load("player.png"),
    //         transform: Transform {
    //             translation: Vec3::new(0., 0., 1.),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(Player { speed: 300.0 })
    //     .insert_bundle(RigidBodyBundle {
    //         mass_properties: (RigidBodyMassPropsFlags::ROTATION_LOCKED).into(),
    //         ..Default::default()
    //     })
    //     .insert_bundle(ColliderBundle {
    //         position: [0., 0.].into(),
    //         shape: ColliderShape::cuboid(collider_size_x, collider_size_y).into(),
    //         ..Default::default()
    //     })
    //     .insert(ColliderPositionSync::Discrete)
    //     .insert(ColliderDebugRender::with_id(1));

    // commands.spawn_bundle(SpriteBundle {
    //     texture: asset_server.load("environment.png"),
    //     transform: Transform {
    //         translation: [-150., 0., 0.].into(),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // });

    let coordinates = (0, 0);
    let dimensions = (11, 4);
    add_collider(commands, coordinates, dimensions);
}

fn add_collider(mut commands: Commands, coordinates: (usize, usize), dimensions: (usize, usize)) {
    let (x_pos, y_pos) = coordinates;
    let (width, height) = dimensions;
    let width = width as f32;
    let height = height as f32;

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
