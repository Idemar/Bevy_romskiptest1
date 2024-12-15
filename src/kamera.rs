use bevy::prelude::*;
use crate::gyte_romskip;

const KAMERA_DISTANSE: f32 = 80.0;

pub struct KameraPlugin;

impl Plugin for KameraPlugin {
    fn bygg(&self, app: App) {
        app.add_systems(Startup, spawn_romskip);
    }
}

fn spawn_kamera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, KAMERA_DISTANSE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}