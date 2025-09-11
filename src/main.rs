use eframe::egui;
use std::collections::HashMap;

mod games;
mod storage;
mod ui;
mod utils;

use games::*;
use storage::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    GameSelection,
    GameConfig(GameType),
    Playing(GameType),
    Results,
    History,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GameType {
    ReadingSpeed,
    WordMemory,
    TextComprehension,
}

impl GameType {
    fn name(&self) -> &str {
        match self {
            GameType::ReadingSpeed => "Lectura Rápida",
            GameType::WordMemory => "Memoria de Palabras",
            GameType::TextComprehension => "Comprensión de Texto",
        }
    }
    
    fn description(&self) -> &str {
        match self {
            GameType::ReadingSpeed => "Lee el texto en el tiempo configurado y después escríbelo",
            GameType::WordMemory => "Memoriza las palabras que aparecen y después escríbelas",
            GameType::TextComprehension => "Lee el texto y responde las preguntas",
        }
    }
}

pub struct SuperlecturaApp {
    state: AppState,
    storage: GameStorage,
    current_game: Option<Box<dyn Game>>,
    current_result: Option<GameResult>,
    game_configs: HashMap<GameType, GameConfig>,
}

impl Default for SuperlecturaApp {
    fn default() -> Self {
        let mut configs = HashMap::new();
        configs.insert(GameType::ReadingSpeed, GameConfig::default());
        configs.insert(GameType::WordMemory, GameConfig::default());
        configs.insert(GameType::TextComprehension, GameConfig::default());
        
        Self {
            state: AppState::GameSelection,
            storage: GameStorage::new(),
            current_game: None,
            current_result: None,
            game_configs: configs,
        }
    }
}

impl eframe::App for SuperlecturaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &self.state {
                AppState::GameSelection => {
                    self.show_game_selection(ui);
                }
                AppState::GameConfig(game_type) => {
                    self.show_game_config(ui, game_type.clone());
                }
                AppState::Playing(game_type) => {
                    self.show_game_play(ui, game_type.clone(), ctx);
                }
                AppState::Results => {
                    self.show_results(ui);
                }
                AppState::History => {
                    self.show_history(ui);
                }
            }
        });
        
        if let Some(game) = &self.current_game {
            if game.needs_repaint() {
                ctx.request_repaint();
            }
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