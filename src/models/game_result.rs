//! Resultados de juegos

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use super::{GameType, GameMode, ExposureTime};

/// Resultado de una partida
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub game_type: GameType,
    pub game_mode: GameMode,
    pub score: f32,
    pub details: GameDetails,
    pub timestamp: SystemTime,
}

/// Detalles específicos según el tipo de juego
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameDetails {
    Matrices {
        matrices_correct: usize,
        total_matrices: usize,
        cells_per_matrix: usize,
        points: usize,
        attempt_number: usize,
    },
    Binarios {
        digits_memorized: usize,
        exposure_time: ExposureTime,
        half_digit_bonus: f32,
        attempt_number: usize,
        is_best: bool,
    },
    FigurasColores {
        figures_correct: usize,
        total_figures: usize,
        initial_speed_ms: u64,
        points: usize,
        attempt_number: usize,
    },
    Decimales {
        digits_memorized: usize,
        exposure_time: ExposureTime,
        half_digit_bonus: f32,
        attempt_number: usize,
        is_best: bool,
    },
    Exhibicion {
        digits_memorized: usize,
        bonus_tenths: f32,
        attempt_number: usize,
    },
}

impl GameDetails {
    /// Calcula la puntuación final considerando bonificaciones
    pub fn calculate_final_score(&self) -> f32 {
        match self {
            GameDetails::Matrices { matrices_correct, cells_per_matrix, .. } => {
                // 10 puntos por casilla correcta
                (*matrices_correct * *cells_per_matrix * 10) as f32
            }
            GameDetails::Binarios { digits_memorized, half_digit_bonus, .. } => {
                *digits_memorized as f32 + *half_digit_bonus
            }
            GameDetails::FigurasColores { figures_correct, .. } => {
                *figures_correct as f32
            }
            GameDetails::Decimales { digits_memorized, half_digit_bonus, .. } => {
                *digits_memorized as f32 + *half_digit_bonus
            }
            GameDetails::Exhibicion { digits_memorized, bonus_tenths, .. } => {
                *digits_memorized as f32 + *bonus_tenths
            }
        }
    }
}

/// Estadísticas de un juego
#[derive(Debug, Clone, Default)]
pub struct GameStats {
    pub total_games: usize,
    pub best_score: f32,
    pub best_result: Option<GameResult>,
}

/// Mejor marca para competición
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestScore {
    pub game_type: GameType,
    pub mode: GameMode,
    pub score: f32,
    pub details: GameDetails,
    pub timestamp: SystemTime,
}

/// Sistema de medio dígito
pub struct HalfDigitSystem;

impl HalfDigitSystem {
    /// Calcula la bonificación de medio dígito
    ///
    /// Reglas:
    /// - En prueba de 1s: aplica desde 14 dígitos
    /// - En prueba de 4s: aplica desde 20 dígitos
    /// - Si el competidor:
    ///   1. Consigue una marca base (ej. 14 dígitos)
    ///   2. Y luego falla un intento superior por un solo dígito
    /// - Se le añade +0.5 dígitos a la puntuación final
    pub fn calculate_bonus(
        base_score: usize,
        failed_attempt_length: usize,
        errors_in_failed: usize,
        exposure_time: &ExposureTime,
    ) -> f32 {
        let threshold = exposure_time.half_digit_threshold();

        // Solo aplica si la marca base supera el umbral
        if base_score < threshold {
            return 0.0;
        }

        // Si falla por exactamente 1 dígito en el intento siguiente
        if errors_in_failed == 1 && failed_attempt_length == base_score + 1 {
            0.5
        } else {
            0.0
        }
    }

    /// Para exhibición: cada fallo por un dígito suma +0.1
    pub fn calculate_exhibition_bonus(errors_by_one_digit: usize) -> f32 {
        errors_by_one_digit as f32 * 0.1
    }
}

/// Resultado de un intento individual (para tracking interno)
#[derive(Debug, Clone)]
pub struct AttemptResult {
    pub attempt_number: usize,
    pub sequence_length: usize,
    pub user_answer: String,
    pub correct: bool,
    pub errors: usize,
    pub time_taken: Duration,
}
