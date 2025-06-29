use crate::bullet::{fire_bullet_patterns, move_bullets, read_bullet_spawn_events, Bullet, BulletSpawnEvent};
use crate::player::{check_bullet_player_collision, fire_shot, move_player, move_shot, respawn_invincibility, respawn_player, spawn_player, switch_player_sprite, PlayerContinueEvent, PlayerDeathEvent, PlayerShot, PlayerSystemSet};
use crate::player_stats::{initialize_player_stats, listen_for_player_continue, listen_for_player_death};
use crate::resources::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;
use std::ops::Range;
use std::f32::consts::PI;
use crate::bosses::boss::{spawn_bosses, update_bosses};
use crate::bosses::boss_health_bar::BossDamageEvent;
use crate::effects::{animate_enemy_death_explosions, create_effects_on_enemy_death};
use crate::enemy::{check_for_enemy_death, check_shot_enemy_collision, move_enemies, spawn_enemies, Enemy, EnemyDeathEvent, EnemySystemSet};
use crate::level1::{level1_plugin, FirstLevelState};
use crate::movement_patterns::{is_finished, MovementPatterns};
use crate::testbed::testbed_plugin;

pub const FRAME_BORDER_LEFT: f32 = -353.0;
pub const FRAME_BORDER_TOP: f32 = 266.0;
pub const FRAME_BORDER_RIGHT: f32 = 97.0;
pub const FRAME_BORDER_BOTTOM: f32 = -258.0;

#[derive(Component)]
pub struct GameObject;

#[derive(Component)]
pub struct SpawnTimer(pub Timer);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LevelState {
    #[default]
    None,
    TestBed,
    Level1,
}

#[derive(Resource)]
pub struct ChosenLevel {
    pub level: LevelState,
}

impl Default for ChosenLevel {
    fn default() -> Self {
        Self {
            level: LevelState::None,
        }
    }
}

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::StartingGame), (
            game_setup,
            initialize_player_stats,
        ))
        .add_systems(Update, (
            (
                move_player,
                respawn_player,
                respawn_invincibility,
                switch_player_sprite,
                check_bullet_player_collision,
                listen_for_player_death,
                listen_for_player_continue,
                fire_shot,
                move_shot,
            ).in_set(PlayerSystemSet),
            (
                spawn_enemies,
                spawn_bosses,
                read_bullet_spawn_events,
                move_enemies,
                fire_bullet_patterns,
                update_bosses,
                check_shot_enemy_collision,
                check_for_enemy_death,
            ).in_set(EnemySystemSet),
            animate_sprite,
            move_bullets,
            out_of_bounds_cleanup,
            create_effects_on_enemy_death,
            animate_enemy_death_explosions,
        ).run_if(in_state(GameState::PlayingGame)))
        .add_systems(OnEnter(LevelState::None), reset_levels)
        .add_plugins((
            testbed_plugin,
            level1_plugin,
        ))
        .init_state::<LevelState>()
        .add_event::<PlayerDeathEvent>()
        .add_event::<EnemyDeathEvent>()
        .add_event::<PlayerContinueEvent>()
        .add_event::<BulletSpawnEvent>()
        .add_event::<BossDamageEvent>()
    ;

}

fn game_setup(
    mut commands: Commands,
    sprites: ResMut<Sprites>,
    game_state: ResMut<NextState<GameState>>,
    level_state: ResMut<NextState<LevelState>>,
    chosen_level: Res<ChosenLevel>,
) {
    draw_background(&mut commands, &sprites);
    draw_ui_frame(&mut commands, &sprites);
    spawn_player(&mut commands, &sprites);
    start_game(game_state, level_state, chosen_level);
}

fn draw_background(commands: &mut Commands, sprites: &ResMut<Sprites>) {
    commands.spawn((
        Name::new("Background"),
        sprites.dark_background.clone(),
        Transform::from_xyz(200.0, 200.0, 0.0),
        GameObject,
    ));
}

fn draw_ui_frame(commands: &mut Commands, sprites: &ResMut<Sprites>) {
    commands.spawn((
        Name::new("UI Frame"),
        sprites.frame.clone(),
        Transform::from_xyz(0.0, 0.0, 1.0),
        GameObject,
    ));
    commands.spawn((
        Name::new("PlayerSpellUIText"),
        sprites.player_spell_text.clone(),
        Transform::from_xyz(162.0, 150.0, 1.1)
            .with_scale(Vec3::splat(1.5)),
        GameObject,
    ));

}

fn start_game(
    mut game_state: ResMut<NextState<GameState>>,
    mut level_state: ResMut<NextState<LevelState>>,
    chosen_level: Res<ChosenLevel>
) {
    game_state.set(GameState::PlayingGame);
    level_state.set(chosen_level.level);
}

fn out_of_bounds_cleanup(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    shot_query: Query<(Entity, &Transform), With<PlayerShot>>,
) {
    let boundary_distance: f32 = 100.0;
    let in_bounds_rect = Rect::from_corners(
        Vec2::new(FRAME_BORDER_LEFT - boundary_distance, FRAME_BORDER_TOP + boundary_distance),
        Vec2::new(FRAME_BORDER_RIGHT + boundary_distance, FRAME_BORDER_BOTTOM - boundary_distance),
    );

    fn despawn_if_out_of_bounds(commands: &mut Commands, in_bounds_rect: Rect, entity: Entity, transform: &Transform) {
        if !in_bounds_rect.contains(transform.translation.truncate()) {
            commands.entity(entity).despawn();
        }
    }

    for (entity, transform) in bullet_query.iter() {
        despawn_if_out_of_bounds(&mut commands, in_bounds_rect, entity, transform);
    }
    for (entity, transform) in enemy_query.iter() {
        despawn_if_out_of_bounds(&mut commands, in_bounds_rect, entity, transform);
    }
    for (entity, transform) in shot_query.iter() {
        despawn_if_out_of_bounds(&mut commands, in_bounds_rect, entity, transform);
    }
}

fn reset_levels(
    mut first_level_state: ResMut<NextState<FirstLevelState>>,
) {
    first_level_state.set(FirstLevelState::Inactive);
}

pub fn is_in_playfield(position: Vec2) -> bool {
    position.x >= FRAME_BORDER_LEFT && position.x <= FRAME_BORDER_RIGHT
    && position.y >= FRAME_BORDER_BOTTOM && position.y <= FRAME_BORDER_TOP
}

pub fn angle_to_transform(origin: Transform, target: Transform) -> Rot2 {
    let diff = target.translation.truncate() - origin.translation.truncate();
    Rot2::radians(diff.y.atan2(diff.x))
}

pub const UP_DOWN_MOVEMENT_BRACKET: Range<f32> = 5.0 * PI / 12.0 .. 7.0 * PI / 12.0;
pub fn is_moving_laterally(movement_pattern: Mut<MovementPatterns>, lateral_movement: f32) -> bool {
    !UP_DOWN_MOVEMENT_BRACKET.contains(&lateral_movement.abs()) && !is_finished(&*movement_pattern)
}
