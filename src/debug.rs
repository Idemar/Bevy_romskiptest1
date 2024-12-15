use bevy::prelude::*;
use crate::skriv_ut_posisjon;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn bygg(&self, app: &mut App) {
        app.add_systems(Update, skriv_ut_posisjon);
    }
}

fn skriv_ut_posisjon(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in Query.iter() {
        info!(
            "Entity {:?} er på posisjon {:?},",
            entity, transform.translation
        );
    }
}