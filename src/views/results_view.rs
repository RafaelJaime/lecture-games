//! Vista de resultados

use eframe::egui;
use crate::models::{AppState, GameDetails};
use crate::controllers::AppController;
use crate::utils::time_format::SystemTimeFormat;

/// Renderiza la vista de resultados después de un juego
pub fn render_results(ui: &mut egui::Ui, controller: &mut AppController) {
    if let Some(result) = controller.get_current_result() {
        ui.vertical_centered(|ui| {
            ui.heading("¡Juego Completado!");
            ui.add_space(20.0);
        });
        
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(format!("Juego: {}", result.game_type.name()));
                    ui.label(format!("Puntuación: {:.1}", result.score));
                    ui.label(format!("Fecha: {}", result.timestamp.format_dm_yhm()));
                });
            });
        });
        
        ui.add_space(20.0);
        
        match &result.details {
            GameDetails::ReadingSpeed { words_correct, total_words, .. } => {
                ui.label(format!("Respuestas correctas: {} de {}", words_correct, total_words));
            }
            GameDetails::WordMemory { words_correct, original_words, .. } => {
                ui.label(format!("Palabras recordadas: {} de {}", words_correct, original_words.len()));
            }
            GameDetails::TextComprehension { questions_correct, total_questions, .. } => {
                ui.label(format!("Respuestas correctas: {} de {}", questions_correct, total_questions));
            }
            GameDetails::INumbs { correct, total, .. } => {
                ui.label(format!("Números correctos: {} de {}", correct, total));
            }
        }
        
        ui.add_space(30.0);
        
        let game_type = result.game_type.clone();
        
        ui.horizontal(|ui| {
            if ui.button("Jugar de nuevo").clicked() {
                controller.start_game(game_type);
            }
            
            if ui.button("Menú principal").clicked() {
                controller.set_state(AppState::GameSelection);
                controller.clear_current_result();
            }
        });
    }
}
