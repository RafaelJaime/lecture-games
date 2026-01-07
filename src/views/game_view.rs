//! Vista durante el juego

#![allow(dead_code)]

use eframe::egui;
use crate::controllers::AppController;

/// Renderiza la vista de juego activo
pub fn render_game(ui: &mut egui::Ui, ctx: &egui::Context, controller: &mut AppController) {
    controller.update_current_game(ui, ctx);
}
