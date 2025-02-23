use crate::bullet_patterns::bullet_stream::BulletStream;
use crate::bullet_patterns::BoxedBulletPattern;
use crate::enemy::EnemySpawner;
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, LevelState, SpawnTimer, FRAME_BORDER_TOP, SPAWN_CENTER, SPAWN_TOP};
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::*;
use crate::bosses::boss::BossSpawner;
use crate::level1::Level1State;

pub fn rumia_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(Level1State::Rumia), rumia_setup)
    ;
}

pub fn rumia_setup(mut commands: Commands) {
    let start = Vec2::new(SPAWN_CENTER, SPAWN_TOP);
    let destination = Vec2::new(SPAWN_CENTER + 150.0, FRAME_BORDER_TOP - 100.0);
    let time = 1.5;
    commands.spawn((
        Name::new("RumiaSpawner"),
        BossSpawner {
            name: "Rumia",
            enemy_type: Rumia,
            starting_position: start,
            movement_pattern: BoxedMovementPattern(Box::new(build_move_to(MoveToBuilder {
                start,
                destination,
                time,
            }))),
        },
        SpawnTimer(Timer::from_seconds(1.0, TimerMode::Once)),
        GameObject,
    ));
}