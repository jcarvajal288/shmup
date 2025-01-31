use std::fmt::Debug;
use bevy::{prelude::*};
use crate::GameState;

// implement menu as a vector of MainMenuStates
#[derive(Component)]
enum MainMenuOption {
    PLAY,
    QUIT,
}

#[derive(Resource)]
struct MenuState {
    options: Vec<Entity>,
    selected: usize,
}

const SELECTED_COLOR: Color = Color::srgb(0.9, 0.0, 0.9);
const UNSELECTED_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub fn menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::MENU), main_menu_setup)
        .add_systems(Update, (handle_input, draw))
    ;
}

fn main_menu_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/Super-Cartoon.ttf");
    let text_font = TextFont {
        font: font.clone(),
        font_size: 50.0,
        ..default()
    };
    let text_justification = JustifyText::Center;

    let play_option_id = commands.spawn((
        Name::new("PlayText"),
        StateScoped(GameState::MENU),
        Text2d::new("Play"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        TextColor(SELECTED_COLOR),
        MainMenuOption::PLAY,
    )).id();
    let quit_option_id = commands.spawn((
        Name::new("QuitText"),
        StateScoped(GameState::MENU),
        Text2d::new("Quit"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_xyz(0.0, -50.0, 100.0),
        TextColor(UNSELECTED_COLOR),
        MainMenuOption::QUIT,
    )).id();
    commands.insert_resource(MenuState {
        options: vec![play_option_id, quit_option_id],
        selected: 0
    });
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
    app_exit_events: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_state.selected = if menu_state.selected == 0 { menu_state.options.len() - 1 } else { menu_state.selected - 1 };
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_state.selected = if menu_state.selected == menu_state.options.len() - 1 { 0 } else { menu_state.selected + 1 };
    } else if keyboard_input.just_pressed(KeyCode::KeyZ) {
        run_main_menu_action(menu_state.selected, app_exit_events);
    }
}

fn run_main_menu_action(menu_selected: usize, mut app_exit_events: EventWriter<AppExit>) {
    match menu_selected {
        0 => println!("Run game"),
        1 => { app_exit_events.send(AppExit::Success); },
        _ => {}
    }
}

fn draw(
    menu_state: Res<MenuState>,
    mut text2d_query: Query<(Entity, &mut TextColor)>,
) {
    for mut text_option in text2d_query.iter_mut() {
        let mut text2d = text_option.0;
        let mut text_color = text_option.1;
        if text2d == menu_state.options[menu_state.selected] {
            text_color.0 = SELECTED_COLOR;
        } else {
            text_color.0 = UNSELECTED_COLOR;
        }
    }
}
