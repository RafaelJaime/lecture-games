//! Resultados de juegos

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use super::GameType;

/// Resultado de una partida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub game_type: GameType,
    pub score: f32,
    pub details: GameDetails,
    pub timestamp: SystemTime,
}

/// Detalles específicos según el tipo de juego
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
