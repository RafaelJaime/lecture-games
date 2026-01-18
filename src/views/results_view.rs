//! Vista de resultados

use eframe::egui;
use crate::models::{AppState, GameDetails};
use crate::controllers::AppController;
use crate::utils::time_format::SystemTimeFormat;

/// Renderiza la vista de resultados después de un juego
pub fn render_results(ui: &mut egui::Ui, controller: &mut AppController) {
    if let Some(result) = controller.get_current_result() {
        ui.vertical_centered(|ui| {
            ui.heading("Prueba Completada!");
            ui.add_space(20.0);
        });

        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(format!("Prueba: {}", result.game_type.name()));
                    ui.label(format!("Modo: {}", result.game_mode.name()));
                    ui.label(format!("Puntuacion: {:.1}", result.score));
                    ui.label(format!("Fecha: {}", result.timestamp.format_dm_yhm()));
                });
            });
        });

        ui.add_space(20.0);

        ui.group(|ui| {
            ui.label("Detalles:");
            match &result.details {
                GameDetails::Matrices { matrices_correct, total_matrices, cells_per_matrix, points, attempt_number } => {
                    ui.label(format!("Matrices: {}/{}", matrices_correct, total_matrices));
                    ui.label(format!("Casillas por matriz: {}", cells_per_matrix));
                    ui.label(format!("Puntos totales: {}", points));
                    ui.label(format!("Intento: {}", attempt_number));
                }
                GameDetails::Binarios { digits_memorized, exposure_time, half_digit_bonus, attempt_number, .. } => {
                    ui.label(format!("Digitos memorizados: {}", digits_memorized));
                    ui.label(format!("Tiempo de exposicion: {}", exposure_time.name()));
                    if *half_digit_bonus > 0.0 {
                        ui.label(format!("Bonus medio digito: +{}", half_digit_bonus));
                    }
                    ui.label(format!("Intentos usados: {}", attempt_number));
                }
                GameDetails::FigurasColores { figures_correct, total_figures, initial_speed_ms, attempt_number, .. } => {
                    ui.label(format!("Figuras correctas: {}/{}", figures_correct, total_figures));
                    ui.label(format!("Velocidad inicial: {}ms", initial_speed_ms));
                    ui.label(format!("Intento: {}", attempt_number));
                }
                GameDetails::Decimales { digits_memorized, exposure_time, half_digit_bonus, attempt_number, .. } => {
                    ui.label(format!("Digitos memorizados: {}", digits_memorized));
                    ui.label(format!("Tiempo de exposicion: {}", exposure_time.name()));
                    if *half_digit_bonus > 0.0 {
                        ui.label(format!("Bonus medio digito: +{}", half_digit_bonus));
                    }
                    ui.label(format!("Intentos usados: {}", attempt_number));
                }
                GameDetails::Exhibicion { digits_memorized, bonus_tenths, attempt_number } => {
                    ui.label(format!("Digitos memorizados: {}", digits_memorized));
                    ui.label("Tiempo de exposicion: 0.5 segundos");
                    if *bonus_tenths > 0.0 {
                        ui.label(format!("Bonus acumulado: +{:.1}", bonus_tenths));
                    }
                    ui.label(format!("Intentos usados: {}", attempt_number));
                }
            }
        });

        ui.add_space(30.0);

        let game_type = result.game_type;
        let mode = controller.get_current_mode();

        ui.horizontal(|ui| {
            if ui.button("Jugar de nuevo").clicked() {
                controller.start_game(game_type);
            }

            if ui.button("Menu principal").clicked() {
                controller.set_state(AppState::GameSelection(mode));
                controller.clear_current_result();
            }
        });
    }
}
