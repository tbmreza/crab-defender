use bevy::app::AppExit;
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
    } else if keyboard_input.pressed(KeyCode::Escape) {
        return Some(KeyCode::Escape);
    }
    None
}

pub fn game_over(
    keyboard_input: Res<Input<KeyCode>>,
    sprite_position: Query<(&mut Direction, &mut Transform)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    // XXX: is sprite_position required? because infinite loop doesn't seem to
    // work here
    // loop {
    for (_control, _transform) in &sprite_position {
        match pressed_keycode(&keyboard_input) {
            Some(KeyCode::Escape) => app_exit_events.send(AppExit),
            _ => {}
        }
    }
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
