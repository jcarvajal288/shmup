use bevy::prelude::*;
use crate::enemy::EnemyType;
use crate::enemy::EnemyType::{BlueFairy, Rumia};
use crate::resources::images::Images;

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub next_first: usize,
    pub next_last: usize,
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(Timer);

#[derive(Component, Clone, Default)]
pub struct AnimatedSprite {
    pub sprite: Sprite,
    pub sprite_size: UVec2,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Resource)]
#[derive(Default)]
pub struct Sprites {
    pub dark_background: Sprite,
    pub frame: Sprite,
    pub player_spell_text: Sprite,
    pub life_counter: Sprite,
    pub blue_fang_shot: Sprite,

    pub remilia: AnimatedSprite,
    pub blue_fairy: AnimatedSprite,
    pub red_fairy: AnimatedSprite,
    pub green_fairy: AnimatedSprite,
    pub yellow_fairy: AnimatedSprite,

    pub rumia: AnimatedSprite,

    pub bullet_white_arrow: Sprite,
    pub bullet_blue_rimmed_circle: Sprite,
    pub bullet_small_red_circle: Sprite,
    pub bullet_small_yellow_circle: Sprite,
    pub bullet_small_green_circle: Sprite,
    pub bullet_small_purple_circle: Sprite,
    pub bullet_small_blue_circle: Sprite,

    pub effect_blue_explosion: Sprite,
    pub effect_red_explosion: Sprite,
    pub effect_yellow_explosion: Sprite,
    pub effect_green_explosion: Sprite,
}



pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (mut indices, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == indices.last {
                    next_animation(&mut indices);
                }
                atlas.index = if !(indices.first..indices.last).contains(&atlas.index) {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn load_sprites(
    mut sprites: ResMut<Sprites>,
    images: ResMut<Images>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    sprites.dark_background = Sprite {
        image: images.dark_background.clone(),
        ..Default::default()
    };
    sprites.frame = Sprite {
        image: images.frame.clone(),
        ..Default::default()
    };

    load_sprite_sheet(images.remilia.clone(), &mut sprites.remilia, &mut texture_atlas_layouts, 45, 45, 4, 2, 0, 3);
    load_sprite_sheet(images.fairies.clone(), &mut sprites.blue_fairy, &mut texture_atlas_layouts, 32, 32, 12, 1, 0, 3);
    load_sprite_sheet(images.fairies.clone(), &mut sprites.red_fairy, &mut texture_atlas_layouts, 32, 32, 12, 1, 12, 15);
    load_sprite_sheet(images.fairies.clone(), &mut sprites.green_fairy, &mut texture_atlas_layouts, 32, 32, 12, 1, 24, 27);
    load_sprite_sheet(images.fairies.clone(), &mut sprites.yellow_fairy, &mut texture_atlas_layouts, 32, 32, 12, 1, 36, 39);
    load_sprite_sheet(images.rumia.clone(), &mut sprites.rumia, &mut texture_atlas_layouts, 32, 48, 5, 2, 0, 0);

    sprites.player_spell_text = Sprite {
        image: images.sidebar.clone(),
        rect: Option::from(Rect::new(307.0, 130.0, 343.0, 162.0)),
        ..Default::default()
    };
    sprites.life_counter = Sprite {
        image: images.sidebar.clone(),
        rect: Option::from(Rect::new(368.0, 98.0, 383.0, 113.0)),
        ..Default::default()
    };
    sprites.blue_fang_shot = Sprite {
        image: images.player_accessories.clone(),
        color: Color::srgba(0.8, 0.8, 1.0, 0.5),
        rect: Option::from(Rect::new(62.0, 36.0, 109.0, 42.0)),
        ..Default::default()
    };

    sprites.bullet_white_arrow = get_bullet_sprite(&images, 0, 0, 0, 1, 16);
    sprites.bullet_blue_rimmed_circle = get_bullet_sprite(&images, 0, 0, 5, 2, 16);
    sprites.bullet_small_red_circle = get_bullet_sprite(&images, 0, 240, 1, 0, 8);
    sprites.bullet_small_yellow_circle = get_bullet_sprite(&images, 0, 240, 4, 1, 8);
    sprites.bullet_small_green_circle = get_bullet_sprite(&images, 0, 240, 2, 1, 8);
    sprites.bullet_small_purple_circle = get_bullet_sprite(&images, 0, 240, 3, 0, 8);
    sprites.bullet_small_blue_circle = get_bullet_sprite(&images, 0, 240, 0, 1, 8);

    sprites.effect_red_explosion = Sprite {
        image: images.effects.clone(),
        rect: Option::from(Rect::new(132.0, 18.0, 194.0, 80.0)),
        ..Default::default()
    };
    sprites.effect_blue_explosion = Sprite {
        image: images.effects.clone(),
        rect: Option::from(Rect::new(196.0, 18.0, 258.0, 80.0)),
        ..Default::default()
    };
    sprites.effect_yellow_explosion = Sprite {
        image: images.effects.clone(),
        rect: Option::from(Rect::new(4.0, 82.0, 66.0, 144.0)),
        ..Default::default()
    };
    sprites.effect_green_explosion = Sprite {
        image: images.effects.clone(),
        rect: Option::from(Rect::new(68.0, 82.0, 130.0, 144.0)),
        ..Default::default()
    };
}

fn get_bullet_sprite(images: &ResMut<Images>, origin_x: usize, origin_y: usize, x_coord: usize, y_coord: usize, size: usize) -> Sprite {
    let x = (origin_x + x_coord * size) as f32;
    let y = (origin_y + y_coord * size) as f32;
    let size_f32 = size as f32;
    Sprite {
        image: images.bullets.clone(),
        rect: Option::from(Rect::new(x, y, x + size_f32, y + size_f32)),
        ..Default::default()
    }
}

pub fn set_animation_frames(indices: &mut AnimationIndices, new_first: usize, new_last: usize) {
    indices.first = new_first;
    indices.last = new_last;
    indices.next_first = new_first;
    indices.next_last = new_last;
}

pub fn set_next_animation(indices: &mut AnimationIndices, new_first: usize, new_last: usize) {
    indices.next_first = new_first;
    indices.next_last = new_last;
}

pub fn set_one_off_animation(indices: &mut AnimationIndices, new_first: usize, new_last: usize) {
    indices.next_first = indices.first;
    indices.next_last = indices.last;
    indices.first = new_first;
    indices.last = new_last;
}

fn next_animation(indices: &mut AnimationIndices) {
    indices.first = indices.next_first;
    indices.last = indices.next_last;
}

#[allow(clippy::too_many_arguments)]
fn load_sprite_sheet(
    texture: Handle<Image>,
    animated_sprite: &mut AnimatedSprite,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    sprite_size_x: u32,
    sprite_size_y: u32,
    columns: u32,
    rows: u32,
    first_index: usize,
    last_index: usize,
) {
    let sprite_size = UVec2::new(sprite_size_x, sprite_size_y);
    let layout = TextureAtlasLayout::from_grid(sprite_size, columns, rows, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices {
        first: first_index,
        last: last_index,
        next_first: first_index,
        next_last: last_index,
    };
    animated_sprite.sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
    );
    animated_sprite.sprite_size = sprite_size;
    animated_sprite.animation_indices = animation_indices;
    animated_sprite.animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
}

pub fn get_sprite_for_enemy_type(sprites: &Res<Sprites>, enemy_type: &EnemyType) -> AnimatedSprite {
    match enemy_type {
        BlueFairy => sprites.blue_fairy.clone(),
        Rumia => sprites.rumia.clone(),
    }
}