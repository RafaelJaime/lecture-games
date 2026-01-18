//! Configuración de juegos

use serde::{Deserialize, Serialize};
use std::time::Duration;
use super::{GameMode, ExposureTime};

/// Configuración compartida para todos los juegos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    /// Modo de juego (Competición o Entrenamiento)
    pub mode: GameMode,

    /// Tiempo de exposición (0.5s, 1s, 4s)
    pub exposure_time: ExposureTime,

    /// Cantidad de elementos a memorizar
    pub element_count: usize,

    /// Número de intento actual (para competición)
    pub current_attempt: usize,

    /// Máximo de intentos permitidos
    pub max_attempts: usize,

    /// Tamaño de matriz (filas, columnas) - para Matrices
    pub matrix_size: (usize, usize),

    /// Número de matrices - para Matrices
    pub matrix_count: usize,

    /// Velocidad inicial en ms - para Figuras de Colores
    pub initial_speed_ms: u64,

    /// Si el usuario rellena casilleros después
    pub fill_boxes: bool,

    /// Duración total permitida (para algunas pruebas)
    pub total_duration: Duration,
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
            initial_speed_ms: 1750,
            fill_boxes: true,
            total_duration: Duration::from_secs(60),
        }
    }
}

impl GameConfig {
    /// Crea configuración para modo competición según el tipo de juego
    pub fn for_competition(game_type: &super::GameType) -> Self {
        use super::GameType;

        let mut config = Self::default();
        config.mode = GameMode::Competition;
        config.max_attempts = game_type.competition_attempts();
        config.total_duration = game_type.competition_duration();

        match game_type {
            GameType::Matrices => {
                config.matrix_count = 12;
                config.matrix_size = (4, 4); // El competidor elige
            }
            GameType::Binarios => {
                config.exposure_time = ExposureTime::OneSecond;
                config.element_count = 10; // Empieza con 10 dígitos
            }
            GameType::FigurasColores => {
                config.element_count = 15;
                config.initial_speed_ms = 1750; // Velocidad lenta inicial
            }
            GameType::Decimales => {
                config.exposure_time = ExposureTime::OneSecond;
                config.element_count = 10;
            }
            GameType::Exhibicion => {
                config.exposure_time = ExposureTime::HalfSecond;
                config.element_count = 10;
            }
        }

        config
    }

    /// Crea configuración para modo entrenamiento
    pub fn for_training() -> Self {
        Self::default()
    }
}
