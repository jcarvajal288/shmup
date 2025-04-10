pub mod spell1;

use crate::bosses::boss::{Boss, BossSpawner};
use crate::enemy::EnemyType::Rumia;
use crate::game::{GameObject, SpawnTimer, FRAME_BORDER_TOP, SPAWN_CENTER, SPAWN_TOP};
use crate::level1::FirstLevelState;
use crate::movement_patterns::move_to::{build_move_to, MoveToBuilder};
use crate::movement_patterns::BoxedMovementPattern;
use bevy::prelude::*;
use crate::bosses::rumia::spell1::spell1_plugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum RumiaState {
    #[default]
    Setup,
    Spell1,
}

pub fn rumia_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(FirstLevelState::Rumia), rumia_setup)
        .add_systems(Update, rumia_orchestrator)
        .add_plugins(spell1_plugin)
        .init_state::<RumiaState>()
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

pub fn rumia_orchestrator(
    boss_query: Query<(&Boss, &BoxedMovementPattern)>,
    rumia_state: Res<State<RumiaState>>,
    mut rumia_next_state: ResMut<NextState<RumiaState>>,
) {
    for (_boss, movement_pattern) in boss_query.iter() {
        if *rumia_state.get() == RumiaState::Setup && movement_pattern.0.is_finished() {
            rumia_next_state.set(RumiaState::Spell1);
            println!("RumiaState set to Spell1");
        }
    }
}