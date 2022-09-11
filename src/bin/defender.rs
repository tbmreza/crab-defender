//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;
use crab_defender::systems::{Direction, *};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(crab_movement)
        .add_system(game_over)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("crab64.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        })
        .insert(Direction::Up);
}
