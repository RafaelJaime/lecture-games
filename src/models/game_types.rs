//! Tipos de juegos y estados de la aplicación

use serde::{Deserialize, Serialize};

/// Modo de juego
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Copy)]
pub enum GameMode {
    /// Competición: reglas estrictas, intentos limitados, mejor marca
    Competition,
    /// Entrenamiento: configurable, sin límites, práctica libre
    #[default]
    Training,
}

impl GameMode {
    pub fn name(&self) -> &str {
        match self {
            GameMode::Competition => "Competición",
            GameMode::Training => "Entrenamiento",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            GameMode::Competition => "Reglas oficiales de competición con intentos limitados",
            GameMode::Training => "Practica libremente con configuración personalizada",
        }
    }
}

/// Estado global de la aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Pantalla inicial de selección de modo
    ModeSelection,
    /// Selección de juego (con modo ya elegido)
    GameSelection(GameMode),
    /// Configuración del juego (solo en Training)
    GameConfig(GameType, GameMode),
    /// Jugando
    Playing(GameType, GameMode),
    /// Resultados
    Results,
    /// Historial
    History,
}

/// Tipos de pruebas de memoria disponibles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub enum GameType {
    /// Memorización visual de matrices (casillas blancas y azules)
    Matrices,
    /// Memorización de secuencias binarias (0 y 1)
    Binarios,
    /// Memorización de figuras geométricas con color
    FigurasColores,
    /// Memorización de secuencias decimales (0-9)
    Decimales,
    /// Prueba de exhibición (0.5 segundos)
    Exhibicion,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::Matrices => "Matrices",
            GameType::Binarios => "Binarios",
            GameType::FigurasColores => "Figuras de Colores",
            GameType::Decimales => "Decimales",
            GameType::Exhibicion => "Exhibición",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            GameType::Matrices => "Memoriza matrices de casillas blancas y azules",
            GameType::Binarios => "Memoriza secuencias de números binarios (0 y 1)",
            GameType::FigurasColores => "Memoriza figuras geométricas con sus colores",
            GameType::Decimales => "Memoriza secuencias de dígitos decimales (0-9)",
            GameType::Exhibicion => "Prueba rápida de exhibición (0.5 segundos)",
        }
    }

    /// Retorna todos los tipos de juegos disponibles
    pub fn all() -> Vec<GameType> {
        vec![
            GameType::Matrices,
            GameType::Binarios,
            GameType::FigurasColores,
            GameType::Decimales,
            GameType::Exhibicion,
        ]
    }

    /// Retorna los intentos permitidos en modo competición
    pub fn competition_attempts(&self) -> usize {
        match self {
            GameType::Matrices => 2,
            GameType::Binarios => 10,
            GameType::FigurasColores => 3,
            GameType::Decimales => 10,
            GameType::Exhibicion => 10,
        }
    }

    /// Retorna la duración total en modo competición
    pub fn competition_duration(&self) -> std::time::Duration {
        use std::time::Duration;
        match self {
            GameType::Matrices => Duration::from_secs(6 * 60), // 6 minutos
            GameType::Binarios => Duration::from_secs(0),       // Sin límite total
            GameType::FigurasColores => Duration::from_secs(3 * 60), // 3 minutos
            GameType::Decimales => Duration::from_secs(0),      // Sin límite total
            GameType::Exhibicion => Duration::from_secs(0),     // Sin límite total
        }
    }
}

/// Estado interno de un juego
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Playing,
    Finished,
    Aborted,
}

/// Tiempo de exposición para pruebas de dígitos
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, Copy)]
pub enum ExposureTime {
    /// 0.5 segundos (exhibición)
    HalfSecond,
    /// 1 segundo
    #[default]
    OneSecond,
    /// 4 segundos
    FourSeconds,
}

impl ExposureTime {
    pub fn as_duration(&self) -> std::time::Duration {
        use std::time::Duration;
        match self {
            ExposureTime::HalfSecond => Duration::from_millis(500),
            ExposureTime::OneSecond => Duration::from_secs(1),
            ExposureTime::FourSeconds => Duration::from_secs(4),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            ExposureTime::HalfSecond => "0.5 segundos",
            ExposureTime::OneSecond => "1 segundo",
            ExposureTime::FourSeconds => "4 segundos",
        }
    }

    /// Umbral para aplicar sistema de medio dígito
    pub fn half_digit_threshold(&self) -> usize {
        match self {
            ExposureTime::HalfSecond => 10, // Exhibición
            ExposureTime::OneSecond => 14,
            ExposureTime::FourSeconds => 20,
        }
    }
}
