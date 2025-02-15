use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::game::{FRAME_BORDER_BOTTOM, FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::GameState;

pub fn game_over_menu_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::GameOver), game_over_menu_setup)
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
    ));
}