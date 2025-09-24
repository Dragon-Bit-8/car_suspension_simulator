use std::io;
use std::{fs::File, io::Write};
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

fn main() {
    // Parámetros físicos
    //Valores por defecto
    let mut m = 300.0; // masa [kg]
    let mut b = 2450.0; // coef. de amortiguamiento [N·s/m]
    let mut k = 20000.0; // constante de resorte [N/m]
    let mut f = 3000.0;

    let mut input = String::new();
    println!("Cuanto vale tu masa?");
    io::stdin().read_line(&mut input).unwrap();
    m = input.trim().parse::<i32>().unwrap() as f32; // masa [kg]
    input.clear();

    println!("Cuanto vale tu tu coeficiente de amortiguamiento?");
    io::stdin().read_line(&mut input).unwrap();
    b = input.trim().parse::<i32>().unwrap() as f32; // coef. de amortiguamiento [N·s/m]
    input.clear();

    println!("Cuanto vale tu constante de resorte?");
    io::stdin().read_line(&mut input).unwrap();
    k = input.trim().parse::<i32>().unwrap() as f32; // constante de resorte [N/m]
    input.clear();

    println!("Cuanta fuerza aplicas?");
    io::stdin().read_line(&mut input).unwrap();
    f = input.trim().parse::<i32>().unwrap() as f32; // fuerza constante [N]
    input.clear();

    println!("Guardar como: ");
    io::stdin().read_line(&mut input).unwrap();

    let mut file = File::create(&format!("{}.csv",input)).unwrap();
    // Condiciones iniciales
    let mut x = 0.0; // posición inicial [m]
    let mut v = 0.0; // velocidad inicial [m/s]

    // Simulación
    let dt = 0.01; // paso de tiempo [s]
    let t_max = 10.0;

    let mut t = 0.0;
    while t <= t_max {
        // Calcular aceleración según la ecuación diferencial
        let a = (f - b * v - k * x) / m;

        // Integración de Euler
        v += a * dt;
        x += v * dt;

        // Guardar posición
        input.push_str(&format!("{}\n", x));

        t += dt;
    }
    file.write(input.as_bytes()).unwrap();
}

/*
use bevy::prelude::*;

#[derive(Resource)]
struct Simulation {
    m: f32,
    b: f32,
    k: f32,
    f: f32,
    x: f32,
    v: f32,
}

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(Simulation {
        m: 1.0,
        b: 2.0,
        k: 5.0,
        f: 20.0,
        x: 0.0,
        v: 0.0,
    })
    .add_systems(Startup, setup)
    //.add_systems(Update, (update_simulation, move_cube))
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    //Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.5, -0.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

// Sistema para actualizar la simulación física
fn update_simulation(time: Res<Time>, mut sim: ResMut<Simulation>) {
    let dt = time.delta_secs();
    let a = (sim.f - sim.b * sim.v - sim.k * sim.x) / sim.m;
    sim.v += a * dt;
    sim.x += sim.v * dt;
}

*/
