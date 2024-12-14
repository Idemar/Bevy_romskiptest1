use bevy::prelude::*;

#[derive(Component,Debug)]
struct Posisjon {
    x: f32,
    y: f32,
}

#[derive(Component, Debug)]
struct Hastighet {
    x: f32,
    y: f32,
}


fn main() {
    App::new()
        .add_systems(Startup, gyte_romskip)
        .add_systems(Update, (oppdatere_posisjon, skriv_ut_posisjon))
        .add_plugins(DefaultPlugins)
        .run();
}

fn gyte_romskip(mut commands: Commands) {
    commands.spawn((Posisjon {x: 0.0, y: 0.0}, Hastighet {x: 1.0, y: 1.0}));
}

fn oppdatere_posisjon(mut query: Query<(&Hastighet, &mut Posisjon)>) {
    for (hastighet, mut posisjon) in query.iter_mut() {
        posisjon.x += hastighet.x;
        posisjon.y += hastighet.y;
    }
}

fn skriv_ut_posisjon(query: Query<(Entity, &Posisjon)>) {
    for (entity, posisjon) in query.iter() {
        info!("Entity {:?} er på posisjon {:?}.", entity, posisjon);
    }
}