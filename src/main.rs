mod kamera;
mod romskip;
mod bevegelse;
mod debug;

use bevy::prelude::*;
use bevy::prelude::Color;
use kamera::KameraPlugin;
use debug::DebugPlugin;
use bevegelse::BevegelsePlugin;
use romskip::RomskipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {color: Color::default(), brightness: 0.75})
        .add_plugins(DefaultPlugins)
        .add_plugins(BevegelsePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(RomskipPlugin)
        .add_plugins(KameraPlugin)
        .run();
}