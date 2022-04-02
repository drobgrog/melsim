use bevy::prelude::*;

#[derive(Component)]
struct Player {}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            width: 1324.,
            height: 1024.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement_system)
        .run();
}

fn sprite_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (_, mut transform) = query.get_single_mut().unwrap();

    let mut horizontal = 0.0;
    let mut vertical = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        horizontal -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        horizontal += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        vertical -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        vertical += 1.0;
    }

    let translation = &mut transform.translation;
    translation.x += horizontal;
    translation.y += vertical;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("environment.png"),
            transform: Transform {
                translation: [-162., 0., 0.1].into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|p| {
            p.spawn_bundle(SpriteBundle {
                texture: asset_server.load("player.png"),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Player {});
        });
}
