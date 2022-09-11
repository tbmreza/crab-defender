//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(crab_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
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

fn pressed_keycode(keyboard_input: &Res<Input<KeyCode>>) -> Option<KeyCode> {
    if keyboard_input.pressed(KeyCode::W) {
        return Some(KeyCode::W);
    } else if keyboard_input.pressed(KeyCode::A) {
        return Some(KeyCode::A);
    } else if keyboard_input.pressed(KeyCode::S) {
        return Some(KeyCode::S);
    } else if keyboard_input.pressed(KeyCode::D) {
        return Some(KeyCode::D);
    }
    None
}

fn crab_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    for (mut control, mut transform) in &mut sprite_position {
        let units = 4.;
        match *control {
            Direction::Up => transform.translation.y += units,
            Direction::Left => transform.translation.x -= units,
            Direction::Down => transform.translation.y -= units,
            Direction::Right => transform.translation.x += units,
        }

        match pressed_keycode(&keyboard_input) {
            Some(KeyCode::W) => *control = Direction::Up,
            Some(KeyCode::A) => *control = Direction::Left,
            Some(KeyCode::S) => *control = Direction::Down,
            Some(KeyCode::D) => *control = Direction::Right,
            _ => {}
        }
    }
}
