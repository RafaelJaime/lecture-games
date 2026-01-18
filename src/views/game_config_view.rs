//! Vista de configuración de juego (modo Entrenamiento)

use eframe::egui;
use crate::models::{GameType, GameMode, GameConfig};
use crate::controllers::AppController;

/// Renderiza la vista de configuración de un juego
pub fn render_game_config(
    ui: &mut egui::Ui,
    controller: &mut AppController,
    game_type: GameType,
    _mode: GameMode,
) {
    let mut config = controller.get_config(&game_type);

    ui.horizontal(|ui| {
        if ui.button("< Volver").clicked() {
            controller.set_state(crate::models::AppState::GameSelection(controller.get_current_mode()));
        }
    });

    ui.add_space(10.0);

    ui.heading(format!("Configurar: {}", game_type.name()));
    ui.separator();
    ui.add_space(20.0);

    match game_type {
        GameType::Decimales1s | GameType::Decimales4s | GameType::Binarios1s | GameType::Binarios4s => {
            render_digits_config(ui, &mut config, game_type);
        }
        GameType::Matrices => {
            render_matrices_config(ui, &mut config);
        }
        GameType::FigurasColores => {
            render_figures_config(ui, &mut config);
        }
    }

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        if ui.button("Iniciar juego").clicked() {
            controller.start_game_with_config(game_type, config.clone());
        }

        if ui.button("Restaurar valores").clicked() {
            config = GameConfig::default();
        }
    });

    // Guardar la configuración actualizada
    controller.set_config(game_type, config);
}

fn render_digits_config(ui: &mut egui::Ui, config: &mut GameConfig, game_type: GameType) {
    // El tiempo de exposición está determinado por el tipo de juego
    let exposure_label = match game_type {
        GameType::Decimales1s | GameType::Binarios1s => "1 segundo",
        GameType::Decimales4s | GameType::Binarios4s => "4 segundos",
        _ => "1 segundo",
    };

    ui.group(|ui| {
        ui.label(format!("Tiempo de exposicion: {} (fijo por tipo de prueba)", exposure_label));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Longitud inicial de secuencia:");
        ui.add(egui::Slider::new(&mut config.element_count, 1..=50).text("digitos"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=20).text("intentos"));
    });

    ui.add_space(10.0);

    ui.collapsing("Configuracion avanzada de visualizacion", |ui| {
        ui.add_space(5.0);

        ui.group(|ui| {
            ui.label("Numero de columnas para mostrar digitos:");
            ui.add(egui::Slider::new(&mut config.digit_columns, 5..=20).text("columnas"));
            ui.label("Los digitos se organizaran en filas de esta longitud");
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Espaciado entre filas:");
            ui.add(egui::Slider::new(&mut config.row_spacing, 5.0..=30.0).text("px"));
        });

        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label("Espaciado entre columnas:");
            ui.add(egui::Slider::new(&mut config.col_spacing, 2.0..=15.0).text("px"));
        });
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.checkbox(&mut config.fast_mode, "Fast mode (correccion automatica)");
        if config.fast_mode {
            ui.label("Las celdas incorrectas se marcaran automaticamente");
        } else {
            ui.label("Modo estandar: escribe la secuencia completa");
        }
    });
}

fn render_matrices_config(ui: &mut egui::Ui, config: &mut GameConfig) {
    ui.group(|ui| {
        ui.label("Tamano de matriz:");
        ui.horizontal(|ui| {
            let mut rows = config.matrix_size.0 as i32;
            let mut cols = config.matrix_size.1 as i32;

            ui.label("Filas:");
            if ui.add(egui::DragValue::new(&mut rows).clamp_range(2..=8)).changed() {
                config.matrix_size.0 = rows as usize;
            }

            ui.label("Columnas:");
            if ui.add(egui::DragValue::new(&mut cols).clamp_range(2..=8)).changed() {
                config.matrix_size.1 = cols as usize;
            }
        });
        ui.label(format!("Total casillas: {}", config.matrix_size.0 * config.matrix_size.1));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de matrices:");
        ui.add(egui::Slider::new(&mut config.matrix_count, 1..=20).text("matrices"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Tiempo de visualizacion por matriz (ms):");
        ui.add(egui::Slider::new(&mut config.matrix_showtime_ms, 1000..=10000).text("ms"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Tiempo en blanco entre matrices (ms):");
        ui.add(egui::Slider::new(&mut config.matrix_blank_time_ms, 0..=2000).text("ms"));
        if config.matrix_blank_time_ms > 0 {
            ui.label("Se mostrara una pantalla en blanco entre matrices");
        } else {
            ui.label("Las matrices se mostraran sin pausa entre ellas");
        }
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=5).text("intentos"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Navegacion durante el input:");
        ui.label("- Flechas izquierda/derecha para cambiar de matriz");
        ui.label("- Teclas 1-9 para ir directamente a una matriz");
    });
}

fn render_figures_config(ui: &mut egui::Ui, config: &mut GameConfig) {
    ui.group(|ui| {
        ui.label("Numero de figuras:");
        ui.add(egui::Slider::new(&mut config.element_count, 5..=30).text("figuras"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Velocidad inicial (milisegundos por figura):");
        ui.add(egui::Slider::new(&mut config.initial_speed_ms, 500..=3000).text("ms"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.checkbox(&mut config.constant_time, "Tiempo constante");
        if config.constant_time {
            ui.label("Todas las figuras se mostraran con la misma velocidad");
        } else {
            ui.label("La velocidad aumentara progresivamente con cada figura");
        }
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Tiempo en blanco entre figuras (ms):");
        ui.add(egui::Slider::new(&mut config.figure_blank_time_ms, 0..=1000).text("ms"));
        if config.figure_blank_time_ms > 0 {
            ui.label("Se mostrara una pantalla en blanco entre figuras");
        } else {
            ui.label("Las figuras se mostraran sin pausa entre ellas");
        }
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=5).text("intentos"));
    });
}
