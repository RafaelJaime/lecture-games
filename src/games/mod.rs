//! Módulo de juegos - Implementaciones de las pruebas de memoria

pub mod decimales;
pub mod binarios;
pub mod exhibicion;
pub mod matrices;
pub mod figuras_colores;

// Re-exportar desde models
pub use crate::models::{GameConfig, GameState, GameResult, GameDetails, ExposureTime};

/// Trait común para todos los juegos
pub trait Game {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn get_state(&self) -> GameState;
    fn get_result(&self) -> Option<GameResult>;
    fn needs_repaint(&self) -> bool { false }
}

/// Botón que también responde a la tecla Enter
pub fn button_with_enter(ui: &mut egui::Ui, text: &str) -> bool {
    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
    ui.button(text).clicked() || enter_pressed
}
