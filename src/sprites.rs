use bevy::prelude::Res;
use crate::enemy::EnemyType;
use crate::enemy::EnemyType::{BlueFairy, Rumia};
use crate::resources::sprites::{AnimatedSprite, Sprites};

pub fn get_sprite_for_enemy_type(sprites: &Res<Sprites>, enemy_type: &EnemyType) -> AnimatedSprite {
    match enemy_type {
        BlueFairy => sprites.blue_fairy.clone(),
        Rumia => sprites.rumia.clone(),
    }
}