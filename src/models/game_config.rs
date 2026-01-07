//! Configuración de juegos

use serde::{Deserialize, Serialize};
use std::time::Duration;
use super::Difficulty;

/// Configuración compartida para todos los juegos
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
