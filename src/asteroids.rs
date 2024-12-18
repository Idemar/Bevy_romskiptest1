use std::ops::Range;

use bevy::prelude::*;
use rand::prelude::*;

use crates::{
    asset_loader::SceneAssets,
    bevegelse::{Akselerasjon, BevegeligObjekt, Hastighet},
    kollisjondeteksjon::kolliderer,
};

const HASTIGHET_SKALERER: f32 = 5.0;
const AKSELERASJON_SKALERER: f32 = 1.0;
const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Y: Range<f32> = 0.0..25.0;
const SPAWN_TIME_SECS: f32 = 1.0;
const ROTASJONS_HASTIGHET: f32 = 2.5;
const RADIUS: f32 = 2.5;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTime {
    timer: Timer,
}

pub struct AsteroidPlugin;

