use bevy::asset::ron::Value;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Hastighet {
    pub value: Vec3,
}

pub struct BevegelsePlugin;

impl Plugin for BevegelsePlugin {
    fn bygg(&self, app: &mut App) {
        app.add_systems(Update, oppdater_posisjon);
    }
}

fn oppdater_posisjon(mut query: Query<(&Hastighet, &mut Transform)>, time: Res<Time>) {
    for (hastighet, mut transform) in Query.iter_mut() {
        transform.translation += hastighet.value * time.delta_seconds();
    }
}