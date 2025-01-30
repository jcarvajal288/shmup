use bevy::prelude::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MENU,
    GAME,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(menu::menu_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

mod menu {
    use bevy::{prelude::*};
    use crate::GameState;

    #[derive(Resource, Default, Clone, Eq, PartialEq)]
    enum MainMenuState {
        #[default]
        PLAY,
        QUIT,
    }

    #[derive(Component)]
    struct PlayMenuOption;

    #[derive(Component)]
    struct QuitMenuOption;

    pub fn menu_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::MENU), main_menu_setup);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let font = asset_server.load("fonts/Super-Cartoon.ttf");
        let text_font = TextFont {
            font: font.clone(),
            font_size: 50.0,
            ..default()
        };
        let text_justification = JustifyText::Center;
        let selected_color = Color::srgb(0.9, 0.0, 0.9);
        let unselected_color = Color::srgb(0.9, 0.9, 0.9);

        commands.spawn((
            Text2d::new("Play"),
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            TextColor(selected_color),
            PlayMenuOption
        ));
        commands.spawn((
            Text2d::new("Quit"),
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            Transform::from_xyz(0.0, -50.0, 100.0),
            TextColor(unselected_color),
            QuitMenuOption
        ));
    }
}
