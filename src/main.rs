mod player_physics;
mod ui;
mod game;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use player_physics::{player_movement, Player};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            width: 1324.,
            height: 1024.,
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
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("environment.png"),
        transform: Transform {
            translation: [-162., 0., 0.0].into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let sprite_size_x = 128.0;
    let sprite_size_y = 192.0;

    // Set the scale
    rapier_config.scale = 64.0;

    // Set gravity
    rapier_config.gravity = [0., 0.].into();

    // Set the size of the collider
    let collider_size_x = sprite_size_x / rapier_config.scale;
    let collider_size_y = sprite_size_y / rapier_config.scale;

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 300.0 })
        .insert_bundle(RigidBodyBundle::default())
        .insert_bundle(ColliderBundle {
            position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);
}
