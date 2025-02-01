use bevy::prelude::*;
use crate::GameState;
use crate::sprite_animation::{animate_sprite, animate_sprite_setup};

pub fn game_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), (game_setup, animate_sprite_setup))
        .add_systems(Update, animate_sprite)
    ;

}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/Super-Cartoon.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    commands.spawn((
        Name::new("GameText"),
        StateScoped(GameState::MENU),
        Text2d::new("Game"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_xyz(0.0, 200.0, 100.0),
        TextColor(Color::WHITE),
    ));
}