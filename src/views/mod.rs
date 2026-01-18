//! Módulo de vistas - Componentes de UI
//!
//! Este módulo contiene todos los componentes visuales y
//! funciones de renderizado de la interfaz.

mod components;
mod mode_selection;
mod menu_view;
mod game_config_view;
mod results_view;
mod history_view;
mod speed_test_view;

#[allow(unused_imports)]
pub use components::*;
pub use mode_selection::*;
pub use menu_view::*;
pub use game_config_view::*;
pub use results_view::*;
pub use history_view::*;
pub use speed_test_view::*;
