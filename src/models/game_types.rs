//! Tipos de juegos y estados de la aplicación

use serde::{Deserialize, Serialize};

/// Estado global de la aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    GameSelection,
    GameConfig(GameType),
    Playing(GameType),
    Results,
    History,
}

/// Tipos de juegos disponibles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameType {
    ReadingSpeed,
    WordMemory,
    TextComprehension,
    INumbs,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::ReadingSpeed => "Memoria Numérica",
            GameType::WordMemory => "Memoria de Palabras",
            GameType::TextComprehension => "Comprensión de Texto",
            GameType::INumbs => "iNumbs (Números / Rellenar casilleros)",
        }
    }
    
    pub fn description(&self) -> &str {
        match self {
            GameType::ReadingSpeed => "Memoriza números y escríbelos después",
            GameType::WordMemory => "Memoriza las palabras que aparecen y después escríbelas",
            GameType::TextComprehension => "Lee el texto y responde las preguntas",
            GameType::INumbs => "Memoriza secuencias numéricas, opcionalmente completa casilleros",
        }
    }
    
    /// Retorna todos los tipos de juegos disponibles
    pub fn all() -> Vec<GameType> {
        vec![
            GameType::ReadingSpeed,
            GameType::WordMemory,
            GameType::TextComprehension,
            GameType::INumbs,
        ]
    }
}

/// Estado interno de un juego
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Finished,
    Aborted,
}

/// Niveles de dificultad
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Difficulty {
    Easy,
    #[default]
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
