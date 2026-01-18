//! Persistencia de datos

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use super::{GameResult, GameConfig, GameType, GameMode, GameStats, BestScore};

/// Almacenamiento persistente de la aplicación
#[derive(Debug, Serialize, Deserialize)]
pub struct GameStorage {
    pub results: Vec<GameResult>,
    pub configs: HashMap<GameType, GameConfig>,
    #[serde(default)]
    pub best_scores: HashMap<(GameType, GameMode), BestScore>,
}

impl GameStorage {
    pub fn new() -> Self {
        if let Ok(loaded) = Self::load() {
            return loaded;
        }

        Self {
            results: Vec::new(),
            configs: HashMap::new(),
            best_scores: HashMap::new(),
        }
    }

    pub fn save_result(&mut self, result: GameResult) {
        // Actualizar mejor marca si aplica
        let key = (result.game_type, result.game_mode);
        let should_update = match self.best_scores.get(&key) {
            Some(best) => result.score > best.score,
            None => true,
        };

        if should_update {
            self.best_scores.insert(key, BestScore {
                game_type: result.game_type,
                mode: result.game_mode,
                score: result.score,
                details: result.details.clone(),
                timestamp: result.timestamp,
            });
        }

        self.results.push(result);
        self.save().ok();
    }

    pub fn save_config(&mut self, game_type: GameType, config: GameConfig) {
        self.configs.insert(game_type, config);
        self.save().ok();
    }

    pub fn get_results_for_game(&self, game_type: &GameType) -> Vec<&GameResult> {
        self.results
            .iter()
            .filter(|r| &r.game_type == game_type)
            .collect()
    }

    pub fn get_results_for_game_and_mode(&self, game_type: &GameType, mode: &GameMode) -> Vec<&GameResult> {
        self.results
            .iter()
            .filter(|r| &r.game_type == game_type && &r.game_mode == mode)
            .collect()
    }

    pub fn get_stats_for_game(&self, game_type: &GameType) -> GameStats {
        let results = self.get_results_for_game(game_type);

        if results.is_empty() {
            return GameStats::default();
        }

        let total_games = results.len();
        let best_score = results.iter().map(|r| r.score).fold(0.0f32, |a, b| a.max(b));
        let best_result = results.iter().max_by(|a, b| {
            a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)
        }).cloned().cloned();

        GameStats {
            total_games,
            best_score,
            best_result,
        }
    }

    pub fn get_best_score(&self, game_type: &GameType, mode: &GameMode) -> Option<&BestScore> {
        self.best_scores.get(&(*game_type, *mode))
    }

    pub fn get_all_results(&self) -> Vec<GameResult> {
        self.results.clone()
    }

    pub fn clear_all_results(&mut self) {
        self.results.clear();
        self.best_scores.clear();
        self.save().ok();
    }

    fn get_save_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("superlectura_games");
        if !path.exists() {
            fs::create_dir_all(&path).ok();
        }
        path.push("save_data.json");
        path
    }

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::get_save_path();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::get_save_path();
        let json = fs::read_to_string(path)?;
        let storage: GameStorage = serde_json::from_str(&json)?;
        Ok(storage)
    }
}

impl Default for GameStorage {
    fn default() -> Self {
        Self::new()
    }
}
