pub mod reading_speed;
pub mod word_memory;
pub mod text_comprehension;
pub mod inumbs;

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
    /// Whether after showing the numbers the game should present input boxes to fill
    /// (used by the iNumbs / retentiva training).
    pub fill_boxes: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            difficulty: Difficulty::Medium,
            duration: Duration::from_secs(30),
            word_count: 100,
            fill_boxes: false,
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
    INumbs {
        correct: usize,
        total: usize,
        time_taken: Duration,
    },
}

pub trait Game {
    fn update(&mut self, ui: &mut egui::Ui, ctx: &egui::Context);
    fn get_state(&self) -> GameState;
    fn get_result(&self) -> Option<GameResult>;
    fn needs_repaint(&self) -> bool { false }
}

pub fn button_with_enter(ui: &mut egui::Ui, text: &str) -> bool {
    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
    ui.button(text).clicked() || enter_pressed
}