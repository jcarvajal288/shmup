use bevy::prelude::*;
use crate::game::{FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::GameState;

#[derive(Component)]
struct MenuShadow;

const SELECTED_COLOR: Color = Color::srgb(0.9, 0.0, 0.9);
const UNSELECTED_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn game_over_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GameOver), game_over_menu_setup)
        .add_systems(Update, (handle_input).run_if(in_state(GameState::GameOver)))
    ;
}

fn game_over_menu_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let shadow_width = FRAME_BORDER_RIGHT - FRAME_BORDER_LEFT + 50.0;
    let shadow_height = FRAME_BORDER_TOP - FRAME_BORDER_BOTTOM + 50.0;
    let frame_center = Vec2::new(-128.0, 4.0);
    let shadow = meshes.add(Rectangle::new(shadow_width, shadow_height));
    commands.spawn((
        Name::new("Game Over Menu Shadow"),
        Mesh2d(shadow),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.75))),
        Transform::from_xyz(frame_center.x, frame_center.y, 0.98),
        MenuShadow,
    ));

    let font = asset_server.load("fonts/Super-Cartoon.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    commands.spawn((
        Name::new("ContinueMenuOption"),
        Text2d::new("Continue"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 100.0, 0.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(SELECTED_COLOR),
    ));
    commands.spawn((
        Name::new("QuitMenuOption"),
        Text2d::new("Quit to Menu"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 125.0, -30.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(UNSELECTED_COLOR),
    ));
}

fn handle_input(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    shadow_query: Query<(Entity, &MenuShadow)>,
) {
    if keyboard_input.pressed(KeyCode::KeyZ) {
        game_state.set(GameState::PlayingGame);
        for (entity, _shadow) in shadow_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}