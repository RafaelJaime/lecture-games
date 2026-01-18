//! Configuración de juegos

use serde::{Deserialize, Serialize};
use std::time::Duration;
use super::{GameMode, ExposureTime, GameType};

/// Configuración compartida para todos los juegos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Modo de juego
    pub mode: GameMode,

    /// Tiempo de exposición (0.5s, 1s, 4s)
    pub exposure_time: ExposureTime,

    /// Cantidad de elementos a memorizar
    pub element_count: usize,

    /// Número de intento actual
    pub current_attempt: usize,

    /// Máximo de intentos permitidos
    pub max_attempts: usize,

    /// Tamaño de matriz (filas, columnas) - para Matrices
    pub matrix_size: (usize, usize),

    /// Número de matrices - para Matrices
    pub matrix_count: usize,

    /// Tiempo de visualización por matriz en ms - para Matrices
    pub matrix_showtime_ms: u64,

    /// Tiempo en blanco entre matrices en ms - para Matrices
    pub matrix_blank_time_ms: u64,

    /// Velocidad inicial en ms - para Figuras de Colores
    pub initial_speed_ms: u64,

    /// Tiempo en blanco entre figuras en ms - para Figuras
    pub figure_blank_time_ms: u64,

    /// Si la velocidad es constante (false = progresiva)
    pub constant_time: bool,

    /// Si el usuario rellena casilleros después
    pub fill_boxes: bool,

    /// Duración total permitida (para algunas pruebas)
    pub total_duration: Duration,

    /// Fast mode: corrección automática con celdas
    pub fast_mode: bool,

    /// Número de columnas para visualización de dígitos
    pub digit_columns: usize,

    /// Separación entre filas
    pub row_spacing: f32,

    /// Separación entre columnas
    pub col_spacing: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mode: GameMode::Training,
            exposure_time: ExposureTime::OneSecond,
            element_count: 10,
            current_attempt: 1,
            max_attempts: 10,
            matrix_size: (4, 4),
            matrix_count: 12,
            matrix_showtime_ms: 5000,
            matrix_blank_time_ms: 1000,
            initial_speed_ms: 1750,
            figure_blank_time_ms: 200,
            constant_time: false,
            fill_boxes: true,
            total_duration: Duration::from_secs(60),
            fast_mode: false,
            digit_columns: 10,
            row_spacing: 10.0,
            col_spacing: 5.0,
        }
    }
}

impl GameConfig {
    /// Crea configuración para modo competición según el tipo de juego
    pub fn for_competition(game_type: &GameType) -> Self {
        let mut config = Self::default();
        config.mode = GameMode::Test;
        config.max_attempts = game_type.competition_attempts();
        config.total_duration = game_type.competition_duration();
        config.exposure_time = game_type.exposure_time();
        config.constant_time = false; // Velocidad progresiva en competición

        match game_type {
            GameType::Matrices => {
                config.matrix_count = 12;
                config.matrix_size = (4, 4);
                config.matrix_showtime_ms = 5000;
                config.matrix_blank_time_ms = 1000;
            }
            GameType::Binarios1s | GameType::Binarios4s => {
                config.element_count = 10;
            }
            GameType::FigurasColores => {
                config.element_count = 15;
                config.initial_speed_ms = 1750;
                config.figure_blank_time_ms = 200;
            }
            GameType::Decimales1s | GameType::Decimales4s => {
                config.element_count = 10;
            }
        }

        config
    }

    /// Crea configuración para modo entrenamiento
    pub fn for_training() -> Self {
        Self::default()
    }

    /// Crea configuración para Exhibition (0.5s)
    pub fn for_exhibition() -> Self {
        let mut config = Self::default();
        config.mode = GameMode::Exhibition;
        config.exposure_time = ExposureTime::HalfSecond;
        config.element_count = 10;
        config
    }
}
