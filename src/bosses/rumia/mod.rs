use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::enemy::EnemySpawner;
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer, FRAME_BORDER_TOP, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::*;

pub fn rumia_setup(mut commands: Commands) {
    let start = Vec2::new(SPAWN_CENTER, SPAWN_TOP);
    let destination = Vec2::new(SPAWN_CENTER, FRAME_BORDER_TOP - 100.0);
    let time = 1.5;
    commands.spawn((
        Name::new("RumiaSpawner"),
        EnemySpawner {
            name: "Rumia",
            enemy_type: Rumia,
            starting_position: start,
            movement_pattern: BoxedMovementPattern(Box::new(build_move_to(MoveToBuilder {
                start,
                destination,
                time,
            }))),
            bullet_pattern: BoxedBulletPattern(Box::new(BulletStream::default())),
        },
        SpawnTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameObject,
    ));
}