pub mod reading_speed;
pub mod word_memory;
pub mod text_comprehension;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::GameType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub time_limit: u32,
    pub difficulty: Difficulty,
    pub custom_text: Option<String>,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            time_limit: 60,
            difficulty: Difficulty::Medium,
            custom_text: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn name(&self) -> &str {
        match self {
            Difficulty::Easy => "Fácil",
            Difficulty::Medium => "Medio",
            Difficulty::Hard => "Difícil",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub id: Uuid,
    pub game_type: GameType,
    pub timestamp: DateTime<Utc>,
    pub score: u32,
    pub accuracy: f32,
    pub time_taken: u32,
    pub config: GameConfig,
    pub details: GameDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameDetails {
    ReadingSpeed {
        original_text: String,
        user_text: String,
        words_correct: u32,
        total_words: u32,
    },
    WordMemory {
        original_words: Vec<String>,
        user_words: Vec<String>,
        words_correct: u32,
    },
    TextComprehension {
        questions_correct: u32,
        total_questions: u32,
        answers: Vec<String>,
    },
}

pub trait Game {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> GameState;
    fn get_result(&self) -> Option<GameResult>;
    fn needs_repaint(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Finished,
    Aborted,
}