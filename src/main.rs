use bevy::prelude::*;
use bevy::core::CorePlugin;
use bevy::input::InputPlugin;
use bevy::window::WindowPlugin;
use bevy::winit::WinitPlugin;
use bevy::render::RenderPlugin;
use bevy::asset::AssetPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            title: String::from("Melbourne Lockdown Simulator"),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_plugin(WinitPlugin::default())
        .add_plugin(AssetPlugin::default())
        .add_plugin(RenderPlugin::default())
        //.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(hello_world_system)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn hello_world_system() {
    println!("hello, world!");
}
