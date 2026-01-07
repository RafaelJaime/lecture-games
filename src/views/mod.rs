//! Módulo de vistas - Componentes de UI
//! 
//! Este módulo contiene todos los componentes visuales y
//! funciones de renderizado de la interfaz.

mod components;
mod menu_view;
mod game_view;
mod results_view;
mod history_view;

// Componentes reutilizables (para uso futuro)
#[allow(unused_imports)]
pub use components::*;
pub use menu_view::*;
#[allow(unused_imports)]
pub use game_view::*;
pub use results_view::*;
pub use history_view::*;
