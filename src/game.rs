use crate::bullet::Bullet;
use crate::enemy::move_enemies;
use crate::images::Images;
use crate::level1::level1_system;
use crate::player::{check_bullet_player_collision, move_player, spawn_player, switch_player_sprite};
use crate::sprites::{animate_sprite, Sprites};
use crate::GameState;
use bevy::prelude::*;

pub const FRAME_BORDER_LEFT: f32 = 32. - 400. + 15.;
pub const FRAME_BORDER_TOP: f32 = 300. - 15. - 19.;
pub const FRAME_BORDER_RIGHT: f32 = 480. - 400. + 17.;
pub const FRAME_BORDER_BOTTOM: f32 = 300. - 560. + 2.;

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (game_setup, level1_system))
        .add_systems(Update, (
            animate_sprite,
            move_player,
            move_bullets,
            switch_player_sprite,
            check_bullet_player_collision,
            move_enemies
        ));

}

fn game_setup(mut commands: Commands, sprites: Res<Sprites>, images: Res<Images>) {
    draw_background(&mut commands, &images);
    draw_ui_frame(&mut commands, &images);
    spawn_player(commands, sprites);
}

fn draw_background(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.dark_background.clone(),
            ..Default::default()
        },
        Transform::from_xyz(200.0, 200.0, 0.0),
    ));
}

fn draw_ui_frame(commands: &mut Commands, images: &Res<Images>) {
    commands.spawn((
        Sprite {
            image: images.frame.clone(),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn move_bullets( // TODO: move this to bullet.rs
    time: Res<Time>,
    mut bullet_query: Query<(&Bullet, &mut Transform)>
) {
    for (bullet, mut transform) in bullet_query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::Y;
        let movement_distance = bullet.speed * time.delta_secs();
        let translation_delta = movement_direction * movement_distance;
        transform.translation += translation_delta;
    }
}

