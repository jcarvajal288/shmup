use std::f32::consts::PI;
use std::time::Duration;
use crate::bullet::{props_for_bullet_type, Bullet};
use crate::game::{PlayerRespawnTimer, FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::sprites::{AnimationIndices, Sprites};
use bevy::math::bounding::{BoundingCircle, IntersectsVolume};
use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerDeathEvent;

#[derive(Component)]
pub struct PlayerShotTimer(Timer);

#[derive(Component)]
pub struct Player {
    pub full_movement_speed: f32,
    pub focused_speed: f32,
    pub hit_circle_radius: f32,
}

#[derive(Component)]
pub struct PlayerShot {
    pub speed: f32,
    pub angle: f32,
    pub damage: i32,
}

pub fn spawn_player(commands: &mut Commands, sprites: &ResMut<Sprites>) {
    commands.spawn((
        Player {
            full_movement_speed: 200.0,
            focused_speed: 60.0,
            hit_circle_radius: 5.0,
        },
        Transform::from_xyz(-128.0, -150.0, 0.5),
        sprites.remilia.clone(),
        sprites.remilia.sprite.clone(),
        PlayerShotTimer(Timer::new(Duration::from_millis(100), TimerMode::Once))
    ));
}

pub fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (player, mut transform) in &mut player_query {
        let speed = if keyboard.pressed(KeyCode::ShiftLeft) { player.focused_speed } else { player.full_movement_speed };
        if keyboard.pressed(KeyCode::ArrowUp) && transform.translation.y < FRAME_BORDER_TOP {
            transform.translation.y += speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowDown) && transform.translation.y > FRAME_BORDER_BOTTOM {
            transform.translation.y -= speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowLeft) && transform.translation.x > FRAME_BORDER_LEFT {
            transform.translation.x -= speed * time.delta_secs();
        }
        if keyboard.pressed(KeyCode::ArrowRight) && transform.translation.x < FRAME_BORDER_RIGHT {
            transform.translation.x += speed * time.delta_secs();
        }
    }
}

pub fn switch_player_sprite(
    mut player_query: Query<(&Player, &mut Sprite, &mut AnimationIndices)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let left_pressed = keyboard.pressed(KeyCode::ArrowLeft);
    let right_pressed = keyboard.pressed(KeyCode::ArrowRight);
    for (_player, mut sprite, mut animation_indices) in &mut player_query {
        if left_pressed && !right_pressed {
            animation_indices.first = 4;
            animation_indices.last = 7;
            sprite.flip_x = false;
        } else if right_pressed && !left_pressed {
            animation_indices.first = 4;
            animation_indices.last = 7;
            sprite.flip_x = true;
        } else {
            animation_indices.first = 0;
            animation_indices.last = 3;
            sprite.flip_x = false;
        }
    }
}
pub fn check_bullet_player_collision(
    mut commands: Commands,
    player_query: Query<(&Player, &Transform, Entity)>,
    bullet_query: Query<(&Bullet, &Transform, Entity)>,
    mut player_death_event_writer:  EventWriter<PlayerDeathEvent>,
) {
    for (player, player_transform, player_entity) in &mut player_query.iter() {
        for (bullet, bullet_transform, bullet_entity) in bullet_query.iter() {
            let bullet_props = props_for_bullet_type(&bullet.bullet_type);
            let player_hit_circle = BoundingCircle::new(player_transform.translation.truncate(), player.hit_circle_radius);
            let bullet_hit_circle = BoundingCircle::new(bullet_transform.translation.truncate(), bullet_props.hit_circle_radius);

            if player_hit_circle.intersects(&bullet_hit_circle) {
                commands.entity(player_entity).despawn();
                commands.entity(bullet_entity).despawn();
                commands.spawn(PlayerRespawnTimer(Timer::from_seconds(0.5, TimerMode::Once)));
                player_death_event_writer.send(PlayerDeathEvent);
            }
        }
    }
}

pub fn respawn_player(
    mut commands: Commands,
    sprites: ResMut<Sprites>,
    time: Res<Time>,
    mut timer_query: Query<&mut PlayerRespawnTimer>,
) {
    for mut timer in &mut timer_query {
        if timer.0.tick(time.delta()).just_finished() {
            spawn_player(&mut commands, &sprites);
        }
    }
}

pub fn fire_shot(
    mut commands: Commands,
    sprites: Res<Sprites>,
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut PlayerShotTimer)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (_player, mut transform, mut shot_timer) in &mut player_query.iter_mut() {
        if shot_timer.0.tick(time.delta()).finished() && keyboard.pressed(KeyCode::KeyZ) {
            let shot_angle = PI / 2.0;
            commands.spawn((
                PlayerShot {
                    speed: 1000.0,
                    angle: shot_angle,
                    damage: 1,
                },
                sprites.blue_fang_shot.clone(),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.4)
                    .with_rotation(Quat::from_rotation_z(shot_angle)),
            ));
            shot_timer.0.reset();
        }
    }
}

pub fn move_shot(
    time: Res<Time>,
    mut shot_query: Query<(&mut PlayerShot, &mut Transform)>,
) {
    for (mut player_shot, mut transform) in &mut shot_query.iter_mut() {
        let movement_direction = Vec3::new(player_shot.angle.cos(), player_shot.angle.sin(), 0.0);
        let movement_distance = player_shot.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    }
}