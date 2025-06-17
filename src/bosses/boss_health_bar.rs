use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::prelude::{Commands, Component, Mesh, Mesh2d, MeshMaterial2d, Query, Rectangle, ResMut, Transform};
use bevy::sprite::ColorMaterial;
use crate::game::{FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::spawns::SPAWN_CENTER;

#[derive(Component)]
pub struct BossHealthBar {
    pub current: i32,
    pub maximum: i32,
}

pub fn spawn_boss_health_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    println!("blah");
    let health_bar_width = FRAME_BORDER_RIGHT - FRAME_BORDER_LEFT;
    let bar_mesh_handle: Handle<Mesh> = meshes.add(Rectangle::new(health_bar_width, 5.0));
    commands.spawn((
        Mesh2d(bar_mesh_handle),
        MeshMaterial2d(materials.add(Color::hsl(1.0, 0.5, 0.5))),
        Transform::from_xyz(SPAWN_CENTER, FRAME_BORDER_TOP, 1.0),
    ));
}