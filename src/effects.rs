use std::f32::consts::PI;
use crate::enemy::{EnemyDeathEvent, EnemyType};
use crate::resources::sprites::Sprites;
use bevy::color::Alpha;
use bevy::math::Quat;
use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, EventReader, Query, Res, Sprite, Time, Transform, Vec3, With};
use rand::Rng;

#[derive(Component)]
pub struct ExplosionEffect;

pub fn create_effects_on_enemy_death(
    mut commands: Commands,
    sprites: Res<Sprites>,
    mut enemy_death_events: EventReader<EnemyDeathEvent>,
) {
    let mut rng = rand::rng();
    for event in enemy_death_events.read() {
        let explosion_sprite = match event.enemy_type {
            EnemyType::BlueFairy => sprites.effect_blue_explosion.clone(),
            EnemyType::Rumia => sprites.effect_blue_explosion.clone(),
        };
        commands.spawn((
            explosion_sprite.clone(),
            Transform::from_translation(event.position).with_scale(Vec3::splat(0.0)),
            ExplosionEffect,
        ));
        let x = rng.random_range(0.5..1.5);
        let y = rng.random_range(0.5..1.5);
        let z = rng.random_range(0.5..1.5);
        commands.spawn((
            explosion_sprite,
            Transform::from_translation(event.position)
                .with_scale(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::new(x, y, z), PI / 4.0)),
            ExplosionEffect,
        ));
    }
}

pub fn animate_enemy_death_explosions(
    mut commands: Commands,
    mut explosion_query: Query<(&mut Sprite, &mut Transform, Entity), With<ExplosionEffect>>,
    time: Res<Time>,
) {
    let fade_speed = 4.0;
    let expand_speed = 8.0;
    for (mut sprite, mut transform, entity) in explosion_query.iter_mut() {
        if sprite.color.alpha() <= 0.0 {
            commands.entity(entity).despawn_recursive();
        } else {
            let new_alpha = sprite.color.alpha() - fade_speed * time.delta_secs();
            sprite.color.set_alpha(new_alpha);
            let new_scale = transform.scale.x + expand_speed * time.delta_secs();
            transform.scale = Vec3::splat(new_scale);
        }
    }
}
