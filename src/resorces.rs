use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct ColorPool {
    pub available: Vec<Color>,
    pub used: Vec<Color>,
}

//##############################################################

#[derive(Resource)]
pub struct Simulation {
    pub m: f32,
    pub b: f32,
    pub k: f32,
    pub f: f32,
    pub x: f32,
    pub v: f32,
}

impl Simulation {
    pub fn new(m: f32, b: f32, k: f32) -> Self {
        let f = m / 4.0 * 9.81;
        Self {
            m,
            b,
            k,
            f,
            x: 0.0,
            v: 0.0,
        }
    }
}

impl std::default::Default for Simulation {
    fn default() -> Self {
        let m = 1200.0;
        let f = m / 4.0 * 9.81;
        Self {
            m,
            b: 2450.0, //2450
            k: 20000.0,
            f,
            x: 0.0,
            v: 0.0,
        }
    }
}

//##############################################################

#[derive(Resource)]
pub struct GraphTimer(pub Timer);

impl GraphTimer {
    pub fn reset_to(&mut self, duration: f32) {
        self.0 = Timer::from_seconds(duration, TimerMode::Once)
    }
}

impl std::default::Default for GraphTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.5, TimerMode::Once))
    }
}
//##############################################################
#[derive(Resource)]
pub struct PositionLog(pub Vec<[f64; 2]>);

impl PositionLog {
    pub fn clear(&mut self) {
        self.0 = vec![[0.0, 0.0]];
    }
}

impl std::default::Default for PositionLog {
    fn default() -> Self {
        Self(vec![[0.0, 0.0]])
    }
}

//##############################################################

#[derive(Debug,PartialEq,Clone)]
pub enum Preset {
    Underdamped,
    CriticallyDamped,
    OverDamped,
    UnDamped
}

impl std::string::ToString for Preset {
    fn to_string(&self) -> String {
        match self {
        Preset::UnDamped => "Oscilatorio".into(),
        Preset::Underdamped => "Sub Amortiguado".into(),
        Preset::CriticallyDamped => "Criticamente Amortiguado".into(),
        Preset::OverDamped => "Sobre Amortiguado".into()
            
        }
    }
}

#[derive(Resource)]
pub struct SimulationModifier {
    pub m: String,
    pub b: String,
    pub k: String,
    pub timer: String,
    pub preset: Preset
}

impl std::default::Default for SimulationModifier {
    fn default() -> Self {
        Self {
            m: String::from("1200"),
            b: String::from("2450.0"),
            k: String::from("20000.0"),
            timer: String::from("2.5"),
            preset: Preset::UnDamped
        }
    }
}
