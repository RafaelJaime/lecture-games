//! Módulo de modelos - Datos y lógica de negocio
//! 
//! Este módulo contiene todas las estructuras de datos y la lógica
//! de negocio de la aplicación, sin dependencias de UI.

mod game_types;
mod game_config;
mod game_result;
mod storage;

pub use game_types::*;
pub use game_config::*;
pub use game_result::*;
pub use storage::*;
