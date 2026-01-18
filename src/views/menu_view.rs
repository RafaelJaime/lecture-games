//! Vista del menú de selección de juegos

use eframe::egui;
use crate::models::{GameType, GameMode, AppState};
use crate::controllers::AppController;

/// Renderiza la vista de selección de juegos
pub fn render_menu(ui: &mut egui::Ui, controller: &mut AppController, mode: GameMode) {
    ui.horizontal(|ui| {
        if ui.button("< Cambiar modo").clicked() {
            controller.go_to_mode_selection();
        }
    });

    ui.add_space(10.0);

    ui.vertical_centered(|ui| {
        ui.heading("Pruebas de Memoria");
        ui.label(format!("Modo: {}", mode.name()));
        ui.add_space(20.0);
    });

    ui.separator();
    ui.add_space(10.0);

    ui.horizontal(|ui| {
        if ui.button("Ver Historial").clicked() {
            controller.set_state(AppState::History);
        }
    });

    ui.add_space(20.0);

    for game_type in GameType::all() {
        render_game_card(ui, controller, &game_type, mode);
        ui.add_space(10.0);
    }
}

/// Renderiza una tarjeta de juego
fn render_game_card(ui: &mut egui::Ui, controller: &mut AppController, game_type: &GameType, mode: GameMode) {
    let stats = controller.get_stats_for_game(game_type);
    let best_score_value = controller.get_best_score(game_type, &mode).map(|b| b.score);

    let frame = egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0));

    let mut should_config = false;
    let mut should_play = false;

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading(game_type.name());
                ui.label(game_type.description());

                ui.add_space(5.0);

                if mode == GameMode::Test || mode == GameMode::GroupsSchools {
                    ui.label(format!(
                        "Intentos permitidos: {}",
                        game_type.competition_attempts()
                    ));
                }

                if let Some(score) = best_score_value {
                    ui.label(format!("Mejor marca: {:.1}", score));
                } else if stats.total_games > 0 {
                    ui.label(format!("Partidas jugadas: {}", stats.total_games));
                } else {
                    ui.label("Sin partidas jugadas");
                }
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Botones de acción
                if mode == GameMode::Training {
                    if ui.button("Configurar").clicked() {
                        should_config = true;
                    }
                }

                if ui.button("Jugar").clicked() {
                    should_play = true;
                }
            });
        });
    });

    if should_config {
        controller.go_to_config(*game_type);
    }
    if should_play {
        controller.start_game(*game_type);
    }
}
