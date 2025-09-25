/*
Subamortiguado 0<ζ<1
Críticamente amortiguado ζ=1
Sobreamortiguado ζ>1
No amortiguado ζ=0

ζ=B/(2√(mK))

SUBAMORTIGUADO:
ζ ≈ 0.5
m = 300 kg
B ≈ 2,450 Ns/m
K = 20,000 N/m

CRITICAMENTE AMORTIGUADO
ζ ≈ 1
m = 300 kg
B ≈ 4,900 Ns/m
K = 20,000 N/m

SOBRE AMORTIGUADO
ζ ≈ 1.5
m = 300 kg
B ≈ 7,350 Ns/m
K = 20,000 N/m

NO AMORTIGUADO
B = 0

FUERZA PROMEDIO
Peso estático por rueda: ~3,000 N
Compresión dinámica típica (baches pequeños): ±1,000 N
Rango total de fuerza: 2,000–4,000 N
*/
mod resorces;
mod components;
mod egui_ui;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use resorces::*;
use components::*;
use egui_ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .insert_resource(Simulation {
            m: 300.0,
            b: 0.0,//2450
            k: 20000.0,
            f: 2943.0,
            x: 0.0,
            v: 0.0,
        })
        .insert_resource(CubeTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_simulation, move_cube).chain())
        .add_systems(EguiPrimaryContextPass, ui_example_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube con marcador
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Cube,
    ));

    // Luz
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Cámara
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.5, -0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Sistema que actualiza la simulación física
fn update_simulation(
    time: Res<Time>,
    mut timer: ResMut<CubeTimer>,
    mut sim: ResMut<Simulation>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let dt = time.delta_secs();
        let a = (sim.f - sim.b * sim.v - sim.k * sim.x) / sim.m;
        sim.v += a * dt;
        sim.x += sim.v * dt;
    }
}

// Sistema que mueve el cubo según la simulación
fn move_cube(sim: Res<Simulation>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.translation.y = sim.x; // Mover en el eje X
    }
}

