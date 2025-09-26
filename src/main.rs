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
mod components;
mod egui_ui;
mod resorces;

use bevy::pbr::wireframe::{Wireframe, WireframeColor, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};

use std::time::Duration;

use components::*;
use egui_ui::*;
use resorces::*;

fn main() {
    let window_plug = WindowPlugin {
        primary_window: Some(Window {
            present_mode: bevy::window::PresentMode::AutoVsync, //Trucazo para forzar 60 fps
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plug))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WireframePlugin::default())
        .insert_resource(Simulation::default())
        .insert_resource(SimulationModifier::default())
        .insert_resource(GraphTimer::default())
        .insert_resource(PositionLog::default())
        .insert_resource(WireframeConfig::default())
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (update_simulation, register_position_log, move_cube)
                .chain()
                .run_if(on_timer(Duration::from_secs_f64(1.0 / 60.0))),
        )
        .add_systems(Update, mark_wireframes)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Coche
    commands.spawn((
        SceneRoot(asset_server.load("3D_Models/Car_Body.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Car,
    ));

    //Llantas
    commands.spawn((
        SceneRoot(asset_server.load("3D_Models/Car_Wheels.glb#Scene0")),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Plano
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2 { x: 100.0, y: 100.0 }))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 0, 0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.insert_resource(ColorPool {
        available: vec![
            Color::srgba_u8(232, 250, 0, 0),
            Color::srgba_u8(255, 230, 2, 0),
            Color::BLACK,
        ],
        used: vec![],
    });

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
        Transform::from_xyz(-7.0, 3.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Sistema que actualiza la simulación física
fn update_simulation(time: Res<Time>, mut sim: ResMut<Simulation>) {
    let dt = time.delta_secs();
    let a = (sim.f - sim.b * sim.v - sim.k * sim.x) / sim.m;
    sim.v += a * dt;
    sim.x += sim.v * dt;
}

// Sistema que mueve el cubo según la simulación
fn move_cube(sim: Res<Simulation>, mut query: Query<&mut Transform, With<Car>>) {
    for mut transform in &mut query {
        transform.translation.y = sim.x; // Mover en el eje X
    }
}

fn register_position_log(
    mut pos_log: ResMut<PositionLog>,
    sim: Res<Simulation>,
    time: Res<Time>,
    mut timer: ResMut<GraphTimer>,
) {
    //timer.0.reset();
    if !timer.0.finished() {
        let t = pos_log.0.last().unwrap()[0] + time.delta_secs() as f64;
        pos_log.0.push([t, sim.x as f64]);
        timer.0.tick(time.delta());
    }
}

fn mark_wireframes(
    mut commands: Commands,
    mut color_pool: ResMut<ColorPool>,
    query: Query<Entity, Added<Mesh3d>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for entity in &query {
        let color = if let Some(c) = color_pool.available.pop() {
            color_pool.used.push(c);
            c
        } else {
            Color::BLACK
        };
        if color != Color::BLACK {
            commands.entity(entity).insert((
                Wireframe,
                WireframeColor { color },
                MeshMaterial3d(materials.add(Color::srgba_u8(0, 0, 0, 0))),
            ));
        } else {
            commands
                .entity(entity)
                .insert((Wireframe, WireframeColor { color }));
        }
    }
}
