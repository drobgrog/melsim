mod ui;
use bevy::prelude::*;

#[derive(Component)]
struct Player {}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(ui::setup_ui)
        .add_startup_system(setup)
        .add_system(sprite_movement_system)
        .run();
}

fn sprite_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (_, mut transform) = query.get_single_mut().unwrap();

    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let translation = &mut transform.translation;
    translation.x += direction;
    translation.x = translation.x.min(380.0).max(-380.0);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    let transform = Transform {
        translation: Vec3::new(0., 0., 0.),
        scale: Vec3::new(0.1, 0.1, 0.1),
        rotation: Default::default(),
    };
    let mut e = commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("test.png"),
        transform,
        ..Default::default()
    });
    e.insert(Player {});
}
