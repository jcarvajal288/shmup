use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut, Default, Clone)]
pub struct AnimationTimer(Timer);

#[derive(Component, Default)]
pub struct AnimatedSprite {
    pub sprite: Sprite,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Resource)]
pub struct Sprites {
    pub remilia: AnimatedSprite,
}

impl Default for Sprites {
    fn default() -> Self {
        Self {
            remilia: AnimatedSprite::default(),
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
                atlas.index = if atlas.index == indices.last {
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("images/remilia.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(45), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 1, last: 3 };
    sprites.remilia.sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
    );
    sprites.remilia.animation_indices = animation_indices;
    sprites.remilia.animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
}
