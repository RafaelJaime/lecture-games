use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::games::{GameResult, GameConfig};
use crate::GameType;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameStorage {
    pub results: Vec<GameResult>,
    pub configs: HashMap<GameType, GameConfig>,
}

impl GameStorage {
    pub fn new() -> Self {
        let storage = Self {
            results: Vec::new(),
            configs: HashMap::new(),
        };
        
        if let Ok(loaded) = Self::load() {
            return loaded;
        }
        
        storage
    }

    pub fn save_result(&mut self, result: GameResult) {
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

    pub fn get_stats_for_game(&self, game_type: &GameType) -> GameStats {
        let results = self.get_results_for_game(game_type);
        
        if results.is_empty() {
            return GameStats::default();
        }
        let total_games = results.len();
        let best_score = results.iter().map(|r| r.score).fold(0.0f32, |a, b| a.max(b));
        
        GameStats {
            total_games,
            best_score,
        }
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

    pub fn get_all_results(&self) -> Vec<GameResult> {
        self.results.clone()
    }

    pub fn clear_all_results(&mut self) {
        self.results.clear();
        self.save().ok();
    }
}

#[derive(Debug, Clone)]
pub struct GameStats {
    pub total_games: usize,
    pub best_score: f32,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            total_games: 0,
            best_score: 0.0,
        }
    }
}