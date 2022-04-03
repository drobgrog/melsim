use bevy::prelude::*;
use bevy_rapier2d::{na::Translation2, prelude::*};

#[derive(Component, Debug, Clone)]
pub enum Pickup {
    Potplant,
}

pub fn pickup_system(mut commands: Commands, narrow_phase: Res<NarrowPhase>) {}
