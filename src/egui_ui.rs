use crate::resorces::*;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui_plot::{Line, Plot, PlotPoints};

//Intefaz gráfica simple
pub fn ui_system(
    mut contexts: EguiContexts,
    mut simulation: ResMut<Simulation>,
    mut pos_log: ResMut<PositionLog>,
    mut simulation_mod: ResMut<SimulationModifier>,
    mut graph_timer: ResMut<GraphTimer>,
) -> Result {
    egui::Window::new("Info").show(contexts.ctx_mut()?, |ui| {
        ui.label(format!("Masa = {}kg", simulation.m));
        ui.label(format!("Coef. Amortiguador = {}Ns/m", simulation.b));
        ui.label(format!("Const. Resorte = {}N/m", simulation.k));
        ui.label(format!("Fuerza constante = {}N", simulation.f));
        ui.label(format!("Velocidad = {}", simulation.v));
        ui.label(format!("Posición = {}", simulation.x));
        ui.label(format!(
            "Razón de amortiguamiento adimensional ( ζ ) = {}",
            (simulation.b) / (2.0) * (simulation.k * simulation.m).sqrt()
        ));
        ui.label(format!(
            "Frecuencia natural ( ω_n ) = {}",
            (simulation.k / simulation.m).sqrt()
        ));
    });

    egui::Window::new("Informacion grafica").show(contexts.ctx_mut()?, |ui| {
        let position = PlotPoints::from(pos_log.0.clone());
        let line = Line::new("Posición", position);
        Plot::new("Grafica de posición")
            .view_aspect(1.0)
            .show(ui, |plot_ui| plot_ui.line(line));
    });

    egui::Window::new("Modificaciones").show(contexts.ctx_mut()?, |ui| {
        let m_mod = simulation_mod.m.trim().parse::<f32>().unwrap_or(0.0);
        let b_mod = simulation_mod.b.trim().parse::<f32>().unwrap_or(0.0);
        let k_mod = simulation_mod.k.trim().parse::<f32>().unwrap_or(0.0);
        let timer_mod = simulation_mod.timer.trim().parse::<f32>().unwrap_or(0.0);
        ui.horizontal(|ui| {
            ui.label("Masa = ");
            ui.text_edit_singleline(&mut simulation_mod.m);
        });
        ui.horizontal(|ui| {
            ui.label("Coef. Amortiguador = ");
            ui.text_edit_singleline(&mut simulation_mod.b);
        });
        ui.horizontal(|ui| {
            ui.label("Const. Resorte = ");
            ui.text_edit_singleline(&mut simulation_mod.k);
        });
        ui.horizontal(|ui| {
            ui.label("Timer de Grafica = ");
            ui.text_edit_singleline(&mut simulation_mod.timer);
        });
        ui.label(format!("Fuerza constante = {}N", m_mod / 4.0 * 9.81));
        ui.label(format!(
            "Razón de amortiguamiento adimensional ( ζ ) = {}",
            (b_mod) / (2.0) * (k_mod * m_mod).sqrt()
        ));
        ui.label(format!(
            "Frecuencia natural ( ω_n ) = {}",
            (k_mod / m_mod).sqrt()
        ));

        if ui.button("Aplicar").clicked() {
            *simulation = Simulation::new(m_mod, b_mod, k_mod);
            graph_timer.reset_to(timer_mod);
            pos_log.clear();
        }
    });
    Ok(())
}
