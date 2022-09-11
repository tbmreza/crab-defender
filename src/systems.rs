use bevy::prelude::*;

#[derive(Component)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
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

pub fn crab_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
) {
    let units = 4.;
    for (mut control, mut transform) in &mut sprite_position {
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
