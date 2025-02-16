use bevy::prelude::*;
use crate::game::{FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::GameState;

#[derive(Component)]
struct MenuShadow;

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
) {
    let shadow_width = FRAME_BORDER_RIGHT - FRAME_BORDER_LEFT + 50.0;
    let shadow_height = FRAME_BORDER_TOP - FRAME_BORDER_BOTTOM + 50.0;
    let frame_center = Vec2::new(-128.0, 4.0);
    let shadow = meshes.add(Rectangle::new(shadow_width, shadow_height));
    commands.spawn((
        Name::new("Game Over Menu Shadow"),
        Mesh2d(shadow),
        MeshMaterial2d(materials.add(Color::srgba(0.0, 0.0, 0.0, 0.75))),
        Transform::from_xyz(frame_center.x, frame_center.y, 0.99),
        MenuShadow,
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