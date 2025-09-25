use bevy::prelude::*;

#[derive(Resource)]
pub struct Simulation {
    pub m: f32,
    pub b: f32,
    pub k: f32,
    pub f: f32,
    pub x: f32,
    pub v: f32,
}

#[derive(Resource)]
pub struct CubeTimer(pub Timer);