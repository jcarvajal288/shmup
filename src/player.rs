use crate::bullet::{props_for_bullet_type, Bullet};
use crate::game::{PlayerRespawnTimer, SpawnTimer, FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::sprites::{AnimationIndices, Sprites};
use bevy::math::bounding::{BoundingCircle, IntersectsVolume};
use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub full_movement_speed: f32,
    pub focused_speed: f32,
    pub hit_circle_radius: f32,
}

pub fn spawn_player(commands: &mut Commands, sprites: &Res<Sprites>) {
    commands.spawn((
        Player {
            full_movement_speed: 200.0,
            focused_speed: 60.0,
            hit_circle_radius: 5.0,
        },
        Transform::from_xyz(-128.0, -150.0, 0.5),
        sprites.remilia.sprite.clone(),
        sprites.remilia.animation_indices.clone(),
        sprites.remilia.animation_timer.clone(),
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
    bullet_query: Query<(&Bullet, &Transform, Entity)>
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
            }
        }
    }
}

