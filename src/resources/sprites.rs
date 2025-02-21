use bevy::prelude::*;
use crate::resources::images::Images;

pub const PLAYER_SPRITE_SIZE: u32 = 45;
pub const FAIRY_SPRITE_SIZE: u32 = 32;

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
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
pub struct Sprites {
    pub dark_background: Sprite,
    pub frame: Sprite,
    pub player_spell_text: Sprite,
    pub life_counter: Sprite,
    pub bullet_white_arrow: Sprite,
    pub blue_fang_shot: Sprite,

    pub remilia: AnimatedSprite,
    pub blue_fairy: AnimatedSprite,
    pub red_fairy: AnimatedSprite,
    pub green_fairy: AnimatedSprite,
    pub yellow_fairy: AnimatedSprite,

    pub rumia: AnimatedSprite,
}

impl Default for Sprites {
    fn default() -> Self {
        Self {
            dark_background: Sprite::default(),
            frame: Sprite::default(),
            remilia: AnimatedSprite::default(),
            blue_fairy: AnimatedSprite::default(),
            red_fairy: AnimatedSprite::default(),
            green_fairy: AnimatedSprite::default(),
            yellow_fairy: AnimatedSprite::default(),
            player_spell_text: Sprite::default(),
            life_counter: Sprite::default(),
            bullet_white_arrow: Sprite::default(),
            blue_fang_shot: Sprite::default(),
            rumia: AnimatedSprite::default(),
        }
    }
}


pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index >= indices.last {
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
    sprites.bullet_white_arrow = Sprite {
        image: images.bullets.clone(),
        rect: Option::from(Rect::new(0.0, 16.0, 16.0, 32.0)),
        ..Default::default()
    };
    sprites.blue_fang_shot = Sprite {
        image: images.player_accessories.clone(),
        color: Color::srgba(0.8, 0.8, 1.0, 0.5),
        rect: Option::from(Rect::new(62.0, 36.0, 109.0, 42.0)),
        ..Default::default()
    };
}

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

    let animation_indices = AnimationIndices { first: first_index, last: last_index };
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