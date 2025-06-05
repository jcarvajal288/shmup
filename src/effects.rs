use std::time::Duration;
use bevy::color::Alpha;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, EventReader, Query, Res, Sprite, Time, TimerMode, Transform};
use bevy::time::Timer;
use crate::enemy::{EnemyDeathEvent, EnemyType};
use crate::resources::sprites::Sprites;

#[derive(Component)]
pub struct ExplosionTimer(Timer);

pub fn create_effects_on_enemy_death(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut enemy_death_events: EventReader<EnemyDeathEvent>,
) {
    for event in enemy_death_events.read() {
        let explosion_sprite = match event.enemy_type {
            EnemyType::BlueFairy => sprites.effect_blue_explosion.clone(),
            EnemyType::Rumia => sprites.effect_blue_explosion.clone(),
        };
        commands.spawn((
            explosion_sprite,
            Transform::from_translation(event.position),
            ExplosionTimer(Timer::new(Duration::from_millis(1000), TimerMode::Once)),
        ));
    }
}

pub fn animate_enemy_death_explosions(
    mut commands: Commands,
    mut explosion_query: Query<(&mut Sprite, &mut ExplosionTimer, Entity)>,
    time: Res<Time>,
) {
    for (mut sprite, mut timer, entity) in explosion_query.iter_mut() {
        if timer.0.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            let new_alpha = sprite.color.alpha() - 2.0 * time.delta_secs();
            sprite.color.set_alpha(new_alpha);
        }
    }
}
