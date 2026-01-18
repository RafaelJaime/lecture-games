//! Vista de selección de modo (Competición / Entrenamiento)

use eframe::egui;
use crate::models::GameMode;
use crate::controllers::AppController;

/// Renderiza la pantalla de selección de modo
pub fn render_mode_selection(ui: &mut egui::Ui, controller: &mut AppController) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.heading("Pruebas de Memoria");
        ui.add_space(40.0);
    });

    ui.separator();
    ui.add_space(30.0);

    ui.vertical_centered(|ui| {
        ui.label("Selecciona el modo de juego:");
        ui.add_space(20.0);
    });

    // 4 modos en grid 2x2
    ui.columns(2, |columns| {
        // Columna 1
        columns[0].vertical_centered(|ui| {
            render_mode_card(
                ui,
                controller,
                GameMode::Training,
                "Training",
                &[
                    "Control total de parametros",
                    "Sin limite de intentos",
                    "Practica libre",
                    "Fast mode disponible",
                ],
            );
            ui.add_space(20.0);
            render_mode_card(
                ui,
                controller,
                GameMode::GroupsSchools,
                "Groups & Schools",
                &[
                    "Modo para grupos",
                    "Proyeccion compartida",
                    "Reglas oficiales",
                    "Speed Test incluido",
                ],
            );
        });

        // Columna 2
        columns[1].vertical_centered(|ui| {
            render_mode_card(
                ui,
                controller,
                GameMode::Test,
                "Test",
                &[
                    "6 pruebas oficiales",
                    "Ejecucion automatica",
                    "Intentos limitados",
                    "Sistema medio digito",
                ],
            );
            ui.add_space(20.0);
            render_mode_card(
                ui,
                controller,
                GameMode::Exhibition,
                "Exhibition",
                &[
                    "Pruebas rapidas (0.5s)",
                    "No oficial",
                    "Demostraciones",
                    "Bonus por decimas",
                ],
            );
        });
    });

    ui.add_space(30.0);
    ui.separator();
    ui.add_space(20.0);

    ui.horizontal(|ui| {
        if ui.button("Ver Historial").clicked() {
            controller.set_state(crate::models::AppState::History);
        }
    });
}

fn render_mode_card(
    ui: &mut egui::Ui,
    controller: &mut AppController,
    mode: GameMode,
    title: &str,
    features: &[&str],
) {
    let frame = egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(20.0))
        .fill(egui::Color32::from_gray(30));

    let response = frame.show(ui, |ui| {
        ui.set_min_width(200.0);
        ui.set_min_height(200.0);

        ui.vertical_centered(|ui| {
            ui.heading(title);
            ui.add_space(15.0);

            for feature in features {
                ui.label(format!("- {}", feature));
            }

            ui.add_space(20.0);

            if ui.button("Seleccionar").clicked() {
                controller.set_mode(mode);
            }
        });
    });

    let response = ui.interact(
        response.response.rect,
        ui.id().with(title),
        egui::Sense::click(),
    );

    if response.hovered() {
        ui.painter().rect_filled(
            response.rect,
            6.0,
            egui::Color32::from_rgba_unmultiplied(100, 100, 200, 30),
        );
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    if response.clicked() {
        controller.set_mode(mode);
    }
}
