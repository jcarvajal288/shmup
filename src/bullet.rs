use bevy::math::Rect;
use bevy::prelude::{Commands, Component, Quat, Res, Sprite, Transform, Vec2};
use crate::images::Images;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}

pub enum BulletType {
    WhiteArrow,
}

pub struct BulletSpawner {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub angle: f32,
    pub speed: f32,
}

pub fn spawn_bullet(commands: &mut Commands, images: &Res<Images>, bullet_spawner: BulletSpawner) {
    commands.spawn((
        get_bullet_sprite(images, bullet_spawner.bullet_type),
        Transform::from_xyz(bullet_spawner.position.x, bullet_spawner.position.y, 0.6)
            .with_rotation(Quat::from_rotation_z(bullet_spawner.angle)),
        Bullet {
            speed: bullet_spawner.speed,
        },
    ));
}

fn get_bullet_sprite(images: &Res<Images>, bullet_type: BulletType) -> Sprite {
    Sprite {
        image: images.bullets.clone(),
        rect: Option::from(rect_for_bullet_type(bullet_type)),
        ..Default::default()
    }
}

fn rect_for_bullet_type(bullet_type: BulletType) -> Rect {
    match bullet_type {
        BulletType::WhiteArrow => Rect::new(0.0, 16.0, 16.0, 32.0),
    }
}