//! Tipos de juegos y estados de la aplicación

use serde::{Deserialize, Serialize};

/// Modo de juego según el README
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default, Copy)]
pub enum GameMode {
    /// Training: Entrenamiento libre con control total de parámetros
    #[default]
    Training,
    /// Test: Ejecuta las 6 pruebas oficiales secuencialmente
    Test,
    /// Groups & Schools: Modo de competición para grupos con proyección
    GroupsSchools,
    /// Exhibition: Pruebas de memoria rápida no oficiales
    Exhibition,
}

impl GameMode {
    pub fn name(&self) -> &str {
        match self {
            GameMode::Training => "Training",
            GameMode::Test => "Test",
            GameMode::GroupsSchools => "Groups & Schools",
            GameMode::Exhibition => "Exhibition",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            GameMode::Training => "Entrenamiento libre con control total de parametros",
            GameMode::Test => "Ejecuta las 6 pruebas oficiales automaticamente",
            GameMode::GroupsSchools => "Competicion para grupos con proyeccion compartida",
            GameMode::Exhibition => "Pruebas de memoria rapida (0.5s) no oficiales",
        }
    }

    /// Indica si el modo requiere Speed Test y datos del competidor
    pub fn requires_competitor_data(&self) -> bool {
        matches!(self, GameMode::Test | GameMode::GroupsSchools | GameMode::Exhibition)
    }

    /// Indica si el modo es secuencial (ejecuta todas las pruebas)
    pub fn is_sequential(&self) -> bool {
        matches!(self, GameMode::Test)
    }
}

/// Datos del competidor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompetitorData {
    pub seat_number: String,
    pub title: String,
    pub name: String,
    pub security_key: String,
}

/// Estado global de la aplicación
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Pantalla inicial de selección de modo
    ModeSelection,
    /// Speed Test (para modos que lo requieren)
    SpeedTest(GameMode),
    /// Datos del competidor (para modos que lo requieren)
    CompetitorInfo(GameMode),
    /// Selección de juego (con modo ya elegido)
    GameSelection(GameMode),
    /// Configuración del juego (solo en Training)
    GameConfig(GameType, GameMode),
    /// Jugando
    Playing(GameType, GameMode),
    /// Resultados
    Results,
    /// Resultados del Test completo (6 pruebas)
    TestResults,
    /// Historial
    History,
}

/// Tipos de pruebas de memoria disponibles
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Copy)]
pub enum GameType {
    /// Memorización visual de matrices (casillas blancas y azules)
    Matrices,
    /// Memorización de secuencias binarias (0 y 1) - 1 segundo
    Binarios1s,
    /// Memorización de secuencias binarias (0 y 1) - 4 segundos
    Binarios4s,
    /// Memorización de figuras geométricas con color
    FigurasColores,
    /// Memorización de secuencias decimales (0-9) - 1 segundo
    Decimales1s,
    /// Memorización de secuencias decimales (0-9) - 4 segundos
    Decimales4s,
}

impl GameType {
    pub fn name(&self) -> &str {
        match self {
            GameType::Matrices => "Matrices",
            GameType::Binarios1s => "Binarios (1s)",
            GameType::Binarios4s => "Binarios (4s)",
            GameType::FigurasColores => "Figuras de Colores",
            GameType::Decimales1s => "Decimales (1s)",
            GameType::Decimales4s => "Decimales (4s)",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            GameType::Matrices => "Memoriza matrices de casillas blancas y azules",
            GameType::Binarios1s => "Secuencias binarias con 1 segundo de exposicion",
            GameType::Binarios4s => "Secuencias binarias con 4 segundos de exposicion",
            GameType::FigurasColores => "Memoriza figuras geometricas con sus colores",
            GameType::Decimales1s => "Secuencias decimales con 1 segundo de exposicion",
            GameType::Decimales4s => "Secuencias decimales con 4 segundos de exposicion",
        }
    }

    /// Retorna todos los tipos de juegos para Training
    pub fn all() -> Vec<GameType> {
        vec![
            GameType::Matrices,
            GameType::Binarios1s,
            GameType::Binarios4s,
            GameType::FigurasColores,
            GameType::Decimales1s,
            GameType::Decimales4s,
        ]
    }

    /// Retorna las 6 pruebas oficiales en orden para Test
    pub fn official_sequence() -> Vec<GameType> {
        vec![
            GameType::Matrices,
            GameType::Binarios1s,
            GameType::Binarios4s,
            GameType::FigurasColores,
            GameType::Decimales1s,
            GameType::Decimales4s,
        ]
    }

    /// Retorna los intentos permitidos en modo competición
    pub fn competition_attempts(&self) -> usize {
        match self {
            GameType::Matrices => 2,
            GameType::Binarios1s | GameType::Binarios4s => 10,
            GameType::FigurasColores => 3,
            GameType::Decimales1s | GameType::Decimales4s => 10,
        }
    }

    /// Retorna la duración total en modo competición
    pub fn competition_duration(&self) -> std::time::Duration {
        use std::time::Duration;
        match self {
            GameType::Matrices => Duration::from_secs(6 * 60), // 6 minutos
            GameType::FigurasColores => Duration::from_secs(3 * 60), // 3 minutos
            _ => Duration::from_secs(0), // Sin límite total
        }
    }

    /// Retorna el tiempo de exposición asociado
    pub fn exposure_time(&self) -> ExposureTime {
        match self {
            GameType::Binarios1s | GameType::Decimales1s => ExposureTime::OneSecond,
            GameType::Binarios4s | GameType::Decimales4s => ExposureTime::FourSeconds,
            _ => ExposureTime::OneSecond,
        }
    }

    /// Indica si es una prueba de dígitos (binarios o decimales)
    pub fn is_digit_test(&self) -> bool {
        matches!(
            self,
            GameType::Binarios1s | GameType::Binarios4s | GameType::Decimales1s | GameType::Decimales4s
        )
    }

    /// Indica si es binario
    pub fn is_binary(&self) -> bool {
        matches!(self, GameType::Binarios1s | GameType::Binarios4s)
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
            ExposureTime::HalfSecond => 10,
            ExposureTime::OneSecond => 14,
            ExposureTime::FourSeconds => 20,
        }
    }
}
