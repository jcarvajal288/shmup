use bevy::math::{Rect, Vec3};
use bevy::prelude::{Commands, Component, Quat, Query, Res, Sprite, Time, Transform, Vec2};
use crate::images::Images;
use crate::movement_patterns::BoxedMovementPattern;

#[derive(Component)]
pub struct Bullet {
    pub bullet_type: BulletType,
    pub speed: f32,
}

pub struct BulletProps {
    pub rect: Rect,
    pub hit_circle_radius: f32,
}

#[derive(Clone, Copy)]
pub enum BulletType {
    WhiteArrow,
}

pub struct BulletSpawner {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub speed: f32,
}

pub fn spawn_bullet(commands: &mut Commands, images: &Res<Images>, bullet_spawner: BulletSpawner) {
    commands.spawn((
        get_bullet_sprite(images, &bullet_spawner.bullet_type),
        Transform::from_xyz(bullet_spawner.position.x, bullet_spawner.position.y, 0.6),
        Bullet {
            speed: bullet_spawner.speed,
            bullet_type: bullet_spawner.bullet_type,
        },
    ));
}

fn get_bullet_sprite(images: &Res<Images>, bullet_type: &BulletType) -> Sprite {
    let props = props_for_bullet_type(bullet_type);
    Sprite {
        image: images.bullets.clone(),
        rect: Option::from(props.rect),
        ..Default::default()
    }
}

pub fn props_for_bullet_type(bullet_type: &BulletType) -> BulletProps {
    match bullet_type {
        BulletType::WhiteArrow => BulletProps {
            rect: Rect::new(0.0, 16.0, 16.0, 32.0),
            hit_circle_radius: 1.0,
        }
    }
}

pub fn move_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&Bullet, &mut Transform, &mut BoxedMovementPattern)>
) {
    for (_bullet, mut transform, mut movement_pattern) in bullet_query.iter_mut() {
        // let movement_direction = transform.rotation * Vec3::Y;
        // let movement_distance = bullet.speed * time.delta_secs();
        // let translation_delta = movement_direction * movement_distance;
        // transform.translation += translation_delta;
        movement_pattern.0.do_move(&mut *transform, &time);

    }
}