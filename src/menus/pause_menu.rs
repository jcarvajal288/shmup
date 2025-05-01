use bevy::prelude::*;
use crate::game::{LevelState, FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::{despawn_screen, GameState};
use crate::menus::{SELECTED_COLOR, UNSELECTED_COLOR};
use crate::player::PlayerContinueEvent;

#[derive(Resource)]
struct PauseMenuState {
    options: Vec<Entity>,
    selected: usize,
}

#[derive(Component)]
struct OnPauseScreen;

pub fn pause_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Paused), pause_menu_setup)
        .add_systems(Update, (handle_input, draw).run_if(in_state(GameState::Paused)))
        .add_systems(OnExit(GameState::Paused), despawn_screen::<OnPauseScreen>)
    ;
}

fn pause_menu_setup(
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
        Name::new("Pause Menu Shadow"),
        Mesh2d(shadow),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.75))),
        Transform::from_xyz(frame_center.x, frame_center.y, 0.98),
        OnPauseScreen,
    ));

    let font = asset_server.load("fonts/Super-Cartoon.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    let continue_option_id = commands.spawn((
        Name::new("ResumeMenuOption"),
        Text2d::new("Resume"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 100.0, 0.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(SELECTED_COLOR),
        OnPauseScreen,
    )).id();
    let restart_option_id = commands.spawn((
        Name::new("RestartMenuOption"),
        Text2d::new("Restart"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 100.0, -30.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(UNSELECTED_COLOR),
        OnPauseScreen,
    )).id();
    let quit_option_id = commands.spawn((
        Name::new("QuitMenuOption"),
        Text2d::new("Quit to Menu"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 100.0, -60.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(UNSELECTED_COLOR),
        OnPauseScreen,
    )).id();
    commands.insert_resource(PauseMenuState {
        options: vec![continue_option_id, restart_option_id, quit_option_id],
        selected: 0
    })
}
fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<PauseMenuState>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_state.selected = if menu_state.selected == 0 { menu_state.options.len() - 1 } else { menu_state.selected - 1 };
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_state.selected = if menu_state.selected == menu_state.options.len() - 1 { 0 } else { menu_state.selected + 1 };
    } else if keyboard_input.pressed(KeyCode::KeyZ) {
        run_menu_action(menu_state.selected, game_state);
    }
}

fn run_menu_action(
    menu_selected: usize,
    mut game_state: ResMut<NextState<GameState>>,
) {
    match menu_selected {
        0 => {
            game_state.set(GameState::PlayingGame);
        },
        1 => {
            game_state.set(GameState::Resetting);
        },
        2 => {
            game_state.set(GameState::MainMenu);
        },
        _ => {}
    }
}
fn draw(
    menu_state: Res<PauseMenuState>,
    mut text2d_query: Query<(Entity, &mut TextColor)>,
) {
    for text_option in text2d_query.iter_mut() {
        let text2d = text_option.0;
        let mut text_color = text_option.1;
        if text2d == menu_state.options[menu_state.selected] {
            text_color.0 = SELECTED_COLOR;
        } else {
            text_color.0 = UNSELECTED_COLOR;
        }
    }
}
