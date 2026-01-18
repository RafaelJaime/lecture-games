use eframe::egui;

// Módulos MVC
mod models;
mod views;
mod controllers;
mod games;
mod utils;

// Re-exportar tipos públicos desde models
pub use models::*;
pub use games::Game;

use controllers::AppController;
use views::{
    render_mode_selection, render_menu, render_game_config, render_results, render_history,
    render_speed_test, render_competitor_info, SpeedTestState, CompetitorInfoState,
};

pub struct SuperlecturaApp {
    controller: AppController,
    speed_test_state: SpeedTestState,
    competitor_info_state: CompetitorInfoState,
}

impl Default for SuperlecturaApp {
    fn default() -> Self {
        Self {
            controller: AppController::default(),
            speed_test_state: SpeedTestState::default(),
            competitor_info_state: CompetitorInfoState::default(),
        }
    }
}

impl eframe::App for SuperlecturaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.controller.get_state().clone() {
                AppState::ModeSelection => {
                    render_mode_selection(ui, &mut self.controller);
                }
                AppState::SpeedTest(mode) => {
                    render_speed_test(ui, &mut self.controller, mode, &mut self.speed_test_state);
                }
                AppState::CompetitorInfo(mode) => {
                    render_competitor_info(ui, &mut self.controller, mode, &mut self.competitor_info_state);
                }
                AppState::GameSelection(mode) => {
                    render_menu(ui, &mut self.controller, mode);
                }
                AppState::GameConfig(game_type, mode) => {
                    render_game_config(ui, &mut self.controller, game_type, mode);
                }
                AppState::Playing(_game_type, _mode) => {
                    self.controller.update_current_game(ui, ctx);
                }
                AppState::Results => {
                    render_results(ui, &mut self.controller);
                }
                AppState::TestResults => {
                    // TODO: Implementar pantalla de resultados del Test completo
                    ui.heading("Resultados del Test");
                    ui.label("Proxima implementacion");
                    if ui.button("Volver al menu").clicked() {
                        self.controller.set_state(AppState::ModeSelection);
                    }
                }
                AppState::History => {
                    render_history(ui, &mut self.controller);
                }
            }
        });

        if self.controller.needs_repaint() {
            ctx.request_repaint();
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Pruebas de Memoria",
        options,
        Box::new(|_cc| Box::new(SuperlecturaApp::default())),
    )
}
