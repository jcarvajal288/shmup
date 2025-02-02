use bevy::prelude::*;

#[derive(Component, Clone, Default)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    load_sprite_sheet("images/remilia.png", &mut sprites.remilia, &asset_server, &mut texture_atlas_layouts);
}

fn load_sprite_sheet(
    filepath: &str,
    animated_sprite: &mut AnimatedSprite,
    asset_server: &AssetServer,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = asset_server.load(filepath);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(45), 4, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animation_indices = AnimationIndices { first: 0, last: 3 };
    animated_sprite.sprite = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
    );
    animated_sprite.animation_indices = animation_indices;
    animated_sprite.animation_timer = AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating));
}
