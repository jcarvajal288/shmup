use bevy::prelude::*;
use crate::GameState;

pub fn level1_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GAME), level1_setup)
    ;

}

fn level1_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        TextColor(Color::WHITE),
    ));
}