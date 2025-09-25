use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::resorces::*;

//Intefaz gráfica simple
pub fn ui_example_system(mut contexts: EguiContexts, simulation: Res<Simulation>) -> Result {
    egui::Window::new("Info").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Masa = {}kg", simulation.m));
        ui.label(format!("Coef. Amortiguador = {}Ns/m", simulation.b));
        ui.label(format!("Const. Resorte = {}N/m", simulation.k));
        ui.label(format!("Fuerza constante = {}N", simulation.f));
        ui.label(format!("Velocidad = {}", simulation.v));
        ui.label(format!("Posición = {}", simulation.x));
        ui.label(format!("Razón de amortiguamiento adimensional (ζ) = {}",(simulation.b)/(2.0)*(simulation.k*simulation.m).sqrt()));
        ui.label(format!("Frecuencia natural (ω_n) = {}",(simulation.k/simulation.m).sqrt()));
    });
    Ok(())
}