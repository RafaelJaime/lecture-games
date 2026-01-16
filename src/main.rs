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
use views::{render_menu, render_results, render_history};

#[derive(Default)]
pub struct SuperlecturaApp {
    controller: AppController,
}

impl eframe::App for SuperlecturaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.controller.get_state().clone() {
                AppState::GameSelection => {
                    render_menu(ui, &mut self.controller);
                }
                AppState::GameConfig(_game_type) => {
                    // Por ahora, ir directamente a jugar
                    render_menu(ui, &mut self.controller);
                }
                AppState::Playing(_game_type) => {
                    self.controller.update_current_game(ui, ctx);
                }
                AppState::Results => {
                    render_results(ui, &mut self.controller);
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
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Superlectura Games",
        options,
        Box::new(|_cc| Box::new(SuperlecturaApp::default())),
    )
}