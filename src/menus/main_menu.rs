use bevy::{prelude::*};
use crate::{despawn_screen, GameState};
use crate::game::{ChosenLevel, LevelState};
use crate::menus::{SELECTED_COLOR, UNSELECTED_COLOR};
use crate::resources::sounds::{PlaySoundEvent, SoundEffect};

// implement menu as a vector of MainMenuStates
#[derive(Resource)]
struct MainMenuState {
    options: Vec<Entity>,
    selected: usize,
}

#[derive(Component)]
struct OnMainMenuScreen;

pub fn main_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::MainMenu), main_menu_setup)
        .add_systems(Update, (handle_input, draw).run_if(in_state(GameState::MainMenu)))
        .add_systems(OnExit(GameState::MainMenu), (
            despawn_screen::<OnMainMenuScreen>,
        ).chain())
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

    let test_option_id = commands.spawn((
        Name::new("TestText"),
        StateScoped(GameState::MainMenu),
        Text2d::new("Test"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        TextColor(SELECTED_COLOR),
        OnMainMenuScreen,
    )).id();
    let play_option_id = commands.spawn((
        Name::new("PlayText"),
        StateScoped(GameState::MainMenu),
        Text2d::new("Play"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_xyz(0.0, -50.0, 100.0),
        TextColor(UNSELECTED_COLOR),
        OnMainMenuScreen,
    )).id();
    let quit_option_id = commands.spawn((
        Name::new("QuitText"),
        StateScoped(GameState::MainMenu),
        Text2d::new("Quit"),
        text_font.clone(),
        TextLayout::new_with_justify(text_justification),
        Transform::from_xyz(0.0, -100.0, 100.0),
        TextColor(UNSELECTED_COLOR),
        OnMainMenuScreen,
    )).id();
    commands.insert_resource(MainMenuState {
        options: vec![test_option_id, play_option_id, quit_option_id],
        selected: 0
    });
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MainMenuState>,
    app_exit_events: EventWriter<AppExit>,
    game_state: ResMut<NextState<GameState>>,
    chosen_level: ResMut<ChosenLevel>,
    mut play_sound_event_writer: EventWriter<PlaySoundEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        menu_state.selected = if menu_state.selected == 0 { menu_state.options.len() - 1 } else { menu_state.selected - 1 };
        play_sound_event_writer.send(PlaySoundEvent(SoundEffect::MenuSelect));
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown) {
        menu_state.selected = if menu_state.selected == menu_state.options.len() - 1 { 0 } else { menu_state.selected + 1 };
        play_sound_event_writer.send(PlaySoundEvent(SoundEffect::MenuSelect));
    } else if keyboard_input.just_pressed(KeyCode::KeyZ) {
        run_main_menu_action(menu_state.selected, app_exit_events, game_state, chosen_level);
        play_sound_event_writer.send(PlaySoundEvent(SoundEffect::MenuSelect));
    }
}

fn run_main_menu_action(
    menu_selected: usize,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameState>>,
    mut chosen_level: ResMut<ChosenLevel>,
) {
    match menu_selected {
        0 => {
            game_state.set(GameState::StartingGame);
            chosen_level.level = LevelState::TestBed;
        },
        1 => {
            game_state.set(GameState::StartingGame);
            chosen_level.level = LevelState::Level1;
        },
        2 => { app_exit_events.send(AppExit::Success); },
        _ => {}
    }
}

fn draw(
    menu_state: Res<MainMenuState>,
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
