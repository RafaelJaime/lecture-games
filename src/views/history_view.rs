//! Vista del historial

use eframe::egui;
use crate::models::{GameType, GameDetails, AppState};
use crate::controllers::AppController;
use crate::utils::time_format::SystemTimeFormat;

/// Renderiza la vista de historial de partidas
pub fn render_history(ui: &mut egui::Ui, controller: &mut AppController) {
    ui.heading("Historial de Partidas");
    ui.separator();
    ui.add_space(10.0);

    if ui.button("< Volver al Menu").clicked() {
        controller.set_state(AppState::ModeSelection);
        return;
    }

    ui.add_space(10.0);

    let results = controller.get_all_results();

    if results.is_empty() {
        ui.label("No hay partidas guardadas");
        return;
    }

    // Mostrar estadísticas generales
    render_general_stats(ui, &results, controller);

    ui.add_space(20.0);

    // Lista de resultados
    render_results_list(ui, &results);

    ui.add_space(20.0);

    // Opciones adicionales
    if ui.button("Limpiar Historial").clicked() {
        controller.clear_all_results();
    }
}

fn render_general_stats(ui: &mut egui::Ui, results: &[crate::models::GameResult], controller: &AppController) {
    ui.collapsing("Resumen General", |ui| {
        let total_games = results.len();
        let total_score: f32 = results.iter().map(|r| r.score).sum();
        let avg_score = if total_games > 0 { total_score / total_games as f32 } else { 0.0 };

        ui.label(format!("Total de partidas: {}", total_games));
        ui.label(format!("Puntuacion promedio: {:.1}", avg_score));

        // Estadísticas por juego
        for game_type in GameType::all() {
            let stats = controller.get_stats_for_game(&game_type);

            if stats.total_games > 0 {
                ui.separator();
                ui.strong(game_type.name());
                ui.label(format!("  Partidas: {}", stats.total_games));
                ui.label(format!("  Mejor puntuacion: {:.1}", stats.best_score));
            }
        }
    });
}

fn render_results_list(ui: &mut egui::Ui, results: &[crate::models::GameResult]) {
    egui::ScrollArea::vertical()
        .max_height(400.0)
        .show(ui, |ui| {
            let mut sorted_results = results.to_vec();
            sorted_results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

            for (i, result) in sorted_results.iter().enumerate() {
                if i > 0 {
                    ui.separator();
                }

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.strong(result.game_type.name());
                            ui.label(format!("Modo: {}", result.game_mode.name()));
                            ui.label(format!("{}", result.timestamp.format_dm_yhm()));
                            ui.label(format!("Puntuacion: {:.1}", result.score));
                        });

                        ui.vertical(|ui| {
                            match &result.details {
                                GameDetails::Matrices { matrices_correct, total_matrices, points, .. } => {
                                    ui.label(format!("Matrices: {}/{}", matrices_correct, total_matrices));
                                    ui.label(format!("Puntos: {}", points));
                                }
                                GameDetails::Binarios { digits_memorized, half_digit_bonus, .. } => {
                                    ui.label(format!("Digitos: {}", digits_memorized));
                                    if *half_digit_bonus > 0.0 {
                                        ui.label(format!("+{} bonus", half_digit_bonus));
                                    }
                                }
                                GameDetails::FigurasColores { figures_correct, total_figures, .. } => {
                                    ui.label(format!("Figuras: {}/{}", figures_correct, total_figures));
                                }
                                GameDetails::Decimales { digits_memorized, half_digit_bonus, .. } => {
                                    ui.label(format!("Digitos: {}", digits_memorized));
                                    if *half_digit_bonus > 0.0 {
                                        ui.label(format!("+{} bonus", half_digit_bonus));
                                    }
                                }
                                GameDetails::Exhibicion { digits_memorized, bonus_tenths, .. } => {
                                    ui.label(format!("Digitos: {}", digits_memorized));
                                    if *bonus_tenths > 0.0 {
                                        ui.label(format!("+{:.1} bonus", bonus_tenths));
                                    }
                                }
                            }
                        });
                    });
                });
            }
        });
}
