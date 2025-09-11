pub mod reading_speed;
pub mod word_memory;
pub mod text_comprehension;

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Medium
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub difficulty: Difficulty,
    pub duration: Duration,
    pub word_count: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Medium,
            duration: Duration::from_secs(30),
            word_count: 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Finished,
    #[allow(dead_code)]
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub game_type: crate::GameType,
    pub score: f32,
    pub details: GameDetails,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameDetails {
    ReadingSpeed { 
        words_correct: usize, 
        total_words: usize,
        time_taken: Duration,
    },
    WordMemory { 
        words_correct: usize, 
        original_words: Vec<String>,
    },
    TextComprehension { 
        questions_correct: usize, 
        total_questions: usize,
    },
}

pub trait Game {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn get_state(&self) -> GameState;
    fn get_result(&self) -> Option<GameResult>;
    fn needs_repaint(&self) -> bool { false }
}