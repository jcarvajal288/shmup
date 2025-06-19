use bevy::asset::{Assets, Handle};
use bevy::color::Color;
use bevy::prelude::{Commands, Component, Event, EventReader, Mesh, Mesh2d, MeshMaterial2d, Query, Rectangle, ResMut, Transform, With};
use bevy::sprite::ColorMaterial;
use crate::game::{FRAME_BORDER_LEFT, FRAME_BORDER_RIGHT, FRAME_BORDER_TOP};
use crate::player::PlayerContinueEvent;
use crate::spawns::SPAWN_CENTER;

#[derive(Component)]
pub struct BossHealthBar {
    pub current: i32,
    pub maximum: i32,
}

#[derive(Component)]
pub struct BossHealthBarBundle;

#[derive(Event)]
pub struct BossDamageEvent(pub i32);

pub fn spawn_boss_health_bar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let health_bar_width = FRAME_BORDER_RIGHT - FRAME_BORDER_LEFT;
    let bar_mesh_handle: Handle<Mesh> = meshes.add(Rectangle::new(health_bar_width, 5.0));
    commands.spawn((
        Mesh2d(bar_mesh_handle),
        MeshMaterial2d(materials.add(Color::hsl(1.0, 0.5, 0.5))),
        Transform::from_xyz(SPAWN_CENTER, FRAME_BORDER_TOP, 1.0),
        BossHealthBarBundle
    ));
}

pub fn scale_boss_health_bar(
    boss_health_bar_query: Query<&BossHealthBar>,
    mut render_bundle_query: Query<&mut Transform, With<BossHealthBarBundle>>,
) {
    for health_bar in boss_health_bar_query.iter() {
        for mut transform in render_bundle_query.iter_mut() {
            //println!("{}/{}", health_bar.current, health_bar.maximum);
            transform.scale.x = health_bar.current as f32 / health_bar.maximum as f32;
        }
    }
}

pub fn listen_for_boss_damage(
    mut boss_damage_event_reader: EventReader<BossDamageEvent>,
    mut boss_health_bar_query: Query<&mut BossHealthBar>,
) {
    for event in boss_damage_event_reader.read() {
        for mut health_bar in boss_health_bar_query.iter_mut() {
            health_bar.current -= event.0;
            if health_bar.current < 0 {
                health_bar.current = 0;
            }
        }
    }
}