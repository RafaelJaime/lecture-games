//! Vista de configuración de juego (modo Entrenamiento)

use eframe::egui;
use crate::models::{GameType, GameMode, GameConfig, ExposureTime};
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
        GameType::Decimales | GameType::Binarios => {
            render_digits_config(ui, &mut config);
        }
        GameType::Exhibicion => {
            render_exhibition_config(ui, &mut config);
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

fn render_digits_config(ui: &mut egui::Ui, config: &mut GameConfig) {
    ui.group(|ui| {
        ui.label("Tiempo de exposicion:");
        ui.horizontal(|ui| {
            ui.radio_value(&mut config.exposure_time, ExposureTime::OneSecond, "1 segundo");
            ui.radio_value(&mut config.exposure_time, ExposureTime::FourSeconds, "4 segundos");
        });
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Longitud inicial de secuencia:");
        ui.add(egui::Slider::new(&mut config.element_count, 1..=30).text("digitos"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=20).text("intentos"));
    });
}

fn render_exhibition_config(ui: &mut egui::Ui, config: &mut GameConfig) {
    ui.group(|ui| {
        ui.label("Tiempo de exposicion: 0.5 segundos (fijo)");
        config.exposure_time = ExposureTime::HalfSecond;
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Longitud inicial de secuencia:");
        ui.add(egui::Slider::new(&mut config.element_count, 1..=20).text("digitos"));
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=20).text("intentos"));
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
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=5).text("intentos"));
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
        ui.label("Nota: La velocidad aumentara progresivamente durante la prueba");
    });

    ui.add_space(10.0);

    ui.group(|ui| {
        ui.label("Numero de intentos:");
        ui.add(egui::Slider::new(&mut config.max_attempts, 1..=5).text("intentos"));
    });
}
