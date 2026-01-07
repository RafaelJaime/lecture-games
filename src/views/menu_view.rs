//! Vista del menú principal

use eframe::egui;
use crate::models::{GameType, AppState};
use crate::controllers::AppController;

/// Renderiza la vista de selección de juegos
pub fn render_menu(ui: &mut egui::Ui, controller: &mut AppController) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("Juegos de Superlectura");
        ui.label("Basados en el libro de Tony Buzan");
        ui.add_space(30.0);
    });
    
    ui.horizontal(|ui| {
        if ui.button("📊 Ver Historial").clicked() {
            controller.set_state(AppState::History);
        }
    });
    
    ui.add_space(20.0);
    ui.separator();
    ui.add_space(20.0);
    
    for game_type in GameType::all() {
        render_game_card(ui, controller, &game_type);
        ui.add_space(10.0);
    }
}

/// Renderiza una tarjeta de juego
fn render_game_card(ui: &mut egui::Ui, controller: &mut AppController, game_type: &GameType) {
    let stats = controller.get_stats_for_game(game_type);
    
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(game_type.name());
                ui.label(game_type.description());
                
                if stats.total_games > 0 {
                    ui.label(format!("Partidas jugadas: {}", stats.total_games));
                    ui.label(format!("Mejor puntuación: {:.1}", stats.best_score));
                } else {
                    ui.label("Sin partidas jugadas");
                }
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Jugar").clicked() {
                    controller.start_game(game_type.clone());
                }
            });
        });
    });
}
