use bevy::prelude::*;
use crate::game::{FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::{despawn_screen, GameState};
use crate::menus::{SELECTED_COLOR, UNSELECTED_COLOR};
use crate::player::PlayerContinueEvent;

#[derive(Resource)]
struct GameOverMenuState {
    options: Vec<Entity>,
    selected: usize,
}

#[derive(Component)]
struct OnGameOverScreen;

pub fn game_over_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GameOver), game_over_menu_setup)
        .add_systems(Update, (handle_input, draw).run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), despawn_screen::<OnGameOverScreen>)
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
        OnGameOverScreen,
    ));

    let font = asset_server.load("fonts/Super-Cartoon.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    let continue_option_id = commands.spawn((
        Name::new("ContinueMenuOption"),
        Text2d::new("Continue"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 100.0, 0.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(SELECTED_COLOR),
        OnGameOverScreen,
    )).id();
    let quit_option_id = commands.spawn((
        Name::new("QuitMenuOption"),
        Text2d::new("Quit to Menu"),
        text_font.clone(),
        Transform::from_xyz(FRAME_BORDER_LEFT + 125.0, -30.0, 0.99),
        TextLayout::new_with_justify(JustifyText::Left),
        TextColor(UNSELECTED_COLOR),
        OnGameOverScreen,
    )).id();
    commands.insert_resource(GameOverMenuState {
        options: vec![continue_option_id, quit_option_id],
        selected: 0
    })
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<GameOverMenuState>,
    player_continue_event_writer: EventWriter<PlayerContinueEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_state.selected = if menu_state.selected == 0 { menu_state.options.len() - 1 } else { menu_state.selected - 1 };
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_state.selected = if menu_state.selected == menu_state.options.len() - 1 { 0 } else { menu_state.selected + 1 };
    } else if keyboard_input.pressed(KeyCode::KeyZ) {
        run_menu_action(menu_state.selected, game_state, player_continue_event_writer);
    }
}

fn run_menu_action(
    menu_selected: usize,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_continue_event_writer: EventWriter<PlayerContinueEvent>,
) {
    match menu_selected {
        0 => {
            game_state.set(GameState::PlayingGame);
            println!("GameState set to PlayingGame");
            player_continue_event_writer.send(PlayerContinueEvent);
        },
        1 => {
            game_state.set(GameState::MainMenu);
            println!("GameState set to MainMenu");
        },
        _ => {}
    }
}
fn draw(
    menu_state: Res<GameOverMenuState>,
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
