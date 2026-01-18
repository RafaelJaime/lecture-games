//! Controlador principal de la aplicación

use std::collections::HashMap;
use eframe::egui;
use crate::models::*;
use crate::games::Game;

/// Controlador principal que maneja el estado y la lógica de la aplicación
pub struct AppController {
    state: AppState,
    storage: GameStorage,
    current_game: Option<Box<dyn Game>>,
    current_result: Option<GameResult>,
    game_configs: HashMap<GameType, GameConfig>,
    current_mode: GameMode,
}

impl AppController {
    pub fn new() -> Self {
        let mut configs = HashMap::new();
        for game_type in GameType::all() {
            configs.insert(game_type, GameConfig::default());
        }

        Self {
            state: AppState::ModeSelection,
            storage: GameStorage::new(),
            current_game: None,
            current_result: None,
            game_configs: configs,
            current_mode: GameMode::Training,
        }
    }

    // === Estado ===

    pub fn get_state(&self) -> &AppState {
        &self.state
    }

    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
    }

    pub fn get_current_mode(&self) -> GameMode {
        self.current_mode
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        self.current_mode = mode;
        self.state = AppState::GameSelection(mode);
    }

    pub fn go_to_mode_selection(&mut self) {
        self.state = AppState::ModeSelection;
    }

    // === Juegos ===

    pub fn start_game(&mut self, game_type: GameType) {
        let config = if self.current_mode == GameMode::Competition {
            GameConfig::for_competition(&game_type)
        } else {
            self.game_configs.get(&game_type).cloned().unwrap_or_default()
        };

        let game: Box<dyn Game> = match game_type {
            GameType::Decimales => {
                Box::new(crate::games::decimales::DecimalesGame::new(config))
            }
            GameType::Binarios => {
                Box::new(crate::games::binarios::BinariosGame::new(config))
            }
            GameType::Exhibicion => {
                Box::new(crate::games::exhibicion::ExhibicionGame::new(config))
            }
            GameType::Matrices => {
                Box::new(crate::games::matrices::MatricesGame::new(config))
            }
            GameType::FigurasColores => {
                Box::new(crate::games::figuras_colores::FigurasColoresGame::new(config))
            }
        };

        self.current_game = Some(game);
        self.state = AppState::Playing(game_type, self.current_mode);
    }

    pub fn start_game_with_config(&mut self, game_type: GameType, config: GameConfig) {
        // Guardar configuración
        self.game_configs.insert(game_type, config.clone());

        let game: Box<dyn Game> = match game_type {
            GameType::Decimales => {
                Box::new(crate::games::decimales::DecimalesGame::new(config))
            }
            GameType::Binarios => {
                Box::new(crate::games::binarios::BinariosGame::new(config))
            }
            GameType::Exhibicion => {
                Box::new(crate::games::exhibicion::ExhibicionGame::new(config))
            }
            GameType::Matrices => {
                Box::new(crate::games::matrices::MatricesGame::new(config))
            }
            GameType::FigurasColores => {
                Box::new(crate::games::figuras_colores::FigurasColoresGame::new(config))
            }
        };

        self.current_game = Some(game);
        self.state = AppState::Playing(game_type, self.current_mode);
    }

    pub fn go_to_config(&mut self, game_type: GameType) {
        self.state = AppState::GameConfig(game_type, self.current_mode);
    }

    pub fn update_current_game(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(game) = &mut self.current_game {
            game.update(ui, ctx);

            match game.get_state() {
                GameState::Playing => {}
                GameState::Finished => {
                    if let Some(result) = game.get_result() {
                        self.storage.save_result(result.clone());
                        self.current_result = Some(result);
                        self.state = AppState::Results;
                        self.current_game = None;
                    }
                }
                GameState::Aborted => {
                    self.state = AppState::GameSelection(self.current_mode);
                    self.current_game = None;
                }
            }
        }
    }

    pub fn needs_repaint(&self) -> bool {
        self.current_game.as_ref().is_some_and(|g| g.needs_repaint())
    }

    // === Resultados ===

    pub fn get_current_result(&self) -> Option<&GameResult> {
        self.current_result.as_ref()
    }

    pub fn clear_current_result(&mut self) {
        self.current_result = None;
    }

    // === Storage ===

    pub fn get_stats_for_game(&self, game_type: &GameType) -> GameStats {
        self.storage.get_stats_for_game(game_type)
    }

    pub fn get_best_score(&self, game_type: &GameType, mode: &GameMode) -> Option<&BestScore> {
        self.storage.get_best_score(game_type, mode)
    }

    pub fn get_all_results(&self) -> Vec<GameResult> {
        self.storage.get_all_results()
    }

    pub fn clear_all_results(&mut self) {
        self.storage.clear_all_results();
    }

    // === Configuración ===

    pub fn get_config(&self, game_type: &GameType) -> GameConfig {
        self.game_configs.get(game_type).cloned().unwrap_or_default()
    }

    pub fn set_config(&mut self, game_type: GameType, config: GameConfig) {
        self.game_configs.insert(game_type, config.clone());
        self.storage.save_config(game_type, config);
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self::new()
    }
}
