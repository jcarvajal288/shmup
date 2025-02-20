use bevy::prelude::*;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::enemy::EnemySpawner;
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer, FRAME_BORDER_TOP, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::BoxedMovementPattern;
use crate::movement_patterns::move_to::MoveTo;

pub fn rumia_setup(mut commands: Commands) {
    commands.spawn((
        Name::new("RumiaSpawner"),
        EnemySpawner {
            name: "Rumia",
            enemy_type: Rumia,
            starting_position: Vec2::new(SPAWN_CENTER, SPAWN_TOP),
            movement_pattern: BoxedMovementPattern(Box::new(MoveTo {
                destination: Vec2::new(SPAWN_CENTER, FRAME_BORDER_TOP - 100.0),
                speed: 100.0,
                acceleration: 0.0,
                ..default()
            })),
            bullet_pattern: BoxedBulletPattern(Box::new(BulletStream::default())),
        },
        SpawnTimer(Timer::from_seconds(0.0, TimerMode::Once)),
        GameObject,
    ));
}