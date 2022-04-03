pub mod covid;
pub mod environment;
mod game;
mod player;
pub mod teleportation;
mod ui;
use crate::covid::covid_system;
use bevy::prelude::*;
use bevy_rapier2d::physics::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use environment::setup_environment;
use player::player_movement;
use player::setup_player;
use teleportation::teleportation_system;

const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_WIDTH: f32 = 1324.0;
pub const TILE_SIZE: f32 = 50.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: String::from("Melbourne Lockdown Simulator"),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            vsync: true,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .init_resource::<game::GameState>()
        .add_startup_system(ui::setup_ui)
        .add_startup_system_to_stage(StartupStage::PreStartup, pre_startup)
        .add_startup_system(setup_player)
        .add_startup_system(setup_environment)
        .add_startup_system(game::setup_state)
        .add_system(player_movement)
        .add_system(covid_system)
        .add_system(ui::update)
        .add_system(ui::update_mental_health_bar_covering)
        .add_system(ui::update_covid_risk)
        .add_system(game::logic)
        .add_system(teleportation_system)
        // .add_plugin(RapierRenderPlugin) // un-comment for a debug view of colliders
        .run();
}

fn pre_startup(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    println!("Setting up physics..");
    // Set the scale
    rapier_config.scale = TILE_SIZE;

    // Set gravity
    rapier_config.gravity = [0., 0.].into();
    println!("..done!");
}
