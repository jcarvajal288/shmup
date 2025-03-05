use crate::movement_patterns::{BoxedMovementPattern};
use crate::resources::sprites::Sprites;
use bevy::prelude::*;
use crate::game::GameObject;

#[derive(Component)]
pub struct Bullet {
    pub bullet_type: BulletType,
}

pub struct BulletProps {
    pub hit_circle_radius: f32,
}

#[derive(Clone, Copy)]
pub enum BulletType {
    WhiteArrow,
    BlueRimmedCircle,
    SmallRedCircle,
    SmallYellowCircle,
    SmallGreenCircle,
    SmallPurpleCircle,
    SmallBlueCircle,
}

pub struct BulletSpawner {
    pub bullet_type: BulletType,
    pub position: Vec2,
    pub movement_pattern: BoxedMovementPattern,
}

pub fn spawn_bullet(commands: &mut Commands, sprites: &Res<Sprites>, bullet_spawner: BulletSpawner) {
    commands.spawn((
        Name::new("Bullet"),
        sprite_for_bullet_type(&bullet_spawner.bullet_type, &sprites),
        Transform::from_xyz(bullet_spawner.position.x, bullet_spawner.position.y, 0.7),
        Bullet {
            bullet_type: bullet_spawner.bullet_type,
        },
        bullet_spawner.movement_pattern,
        GameObject,
    ));
}

pub fn props_for_bullet_type(_bullet_type: &BulletType) -> BulletProps {
    BulletProps {
        hit_circle_radius: 1.0,
    }
}

pub fn move_bullets(
    time: Res<Time>,
    mut bullet_query: Query<(&Bullet, &mut Transform, &mut BoxedMovementPattern)>
) {
    for (_bullet, mut transform, mut movement_pattern) in bullet_query.iter_mut() {
        movement_pattern.0.do_move(&mut *transform, &time);
    }
}

fn sprite_for_bullet_type(bullet_type: &BulletType, sprites: &Sprites) -> Sprite {
    match bullet_type {
        BulletType::WhiteArrow => sprites.bullet_white_arrow.clone(),
        BulletType::BlueRimmedCircle => sprites.bullet_blue_rimmed_circle.clone(),
        BulletType::SmallRedCircle => sprites.bullet_small_red_circle.clone(),
        BulletType::SmallYellowCircle => sprites.bullet_small_yellow_circle.clone(),
        BulletType::SmallGreenCircle => sprites.bullet_small_green_circle.clone(),
        BulletType::SmallPurpleCircle => sprites.bullet_small_purple_circle.clone(),
        BulletType::SmallBlueCircle => sprites.bullet_small_blue_circle.clone(),
    }
}