//! Testing systems doesn't require graphics (e.g. rendering and windowing)
//! plugins. Plus, cargo execute tests in multiple threads, which makes
//! integrating graphics tests not only unnecessary for our purposes but also
//! impossible (with simple infra, maybe).

use bevy::prelude::*;

// use crab_defender::systems::{crab_movement, Direction};
//
// #[test]
// fn keyboard_input_changes_direction() {
//     App::new()
//         .add_plugins(custom_plugins)
//         .add_system(crab_movement)
//         .update();
//
// }

#[derive(Component, Default)]
struct Enemy {
    hit_points: u32,
    score_value: u32,
}

struct EnemyDied(u32);

struct Score(u32);

fn despawn_dead_enemies(
    mut commands: Commands,
    mut dead_enemies: EventWriter<EnemyDied>,
    enemies: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in &enemies {
        if enemy.hit_points == 0 {
            commands.entity(entity).despawn_recursive();
            dead_enemies.send(EnemyDied(enemy.score_value));
        }
    }
}

fn hurt_enemies(mut enemies: Query<&mut Enemy>) {
    for mut enemy in &mut enemies {
        enemy.hit_points -= 1;
    }
}

#[test]
fn did_hurt_enemy() {
    let mut app = App::new();

    app.insert_resource(Score(0));
    app.add_event::<EnemyDied>();
    app.add_system(hurt_enemies.before(despawn_dead_enemies));
    app.add_system(despawn_dead_enemies);

    // Setup test entities
    let enemy_id = app
        .world
        .spawn()
        .insert(Enemy {
            hit_points: 5,
            score_value: 3,
        })
        .id();

    app.update();

    assert!(app.world.get::<Enemy>(enemy_id).is_some());
    assert_eq!(app.world.get::<Enemy>(enemy_id).unwrap().hit_points, 4);
}
