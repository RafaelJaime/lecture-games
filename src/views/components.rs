//! Componentes de UI reutilizables
//!
//! Estos componentes están diseñados para uso futuro cuando
//! se refactorice la UI de los juegos individuales.

#![allow(dead_code)]

use eframe::egui;

/// Botón que también responde a la tecla Enter
/// Devuelve true si se hizo clic o se presionó Enter
pub fn button_with_enter(ui: &mut egui::Ui, text: &str) -> bool {
    let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
    ui.button(text).clicked() || enter_pressed
}

/// Dibuja un botón de menú para volver atrás
/// Retorna true si se hizo clic
pub fn menu_button(ui: &mut egui::Ui) -> bool {
    let mut clicked = false;
    ui.horizontal(|ui| {
        if ui.button("< Menú").clicked() {
            clicked = true;
        }
    });
    ui.separator();
    ui.add_space(10.0);
    clicked
}

/// Dibuja un grupo de instrucciones
pub fn instructions_group(ui: &mut egui::Ui, title: &str, instructions: &[&str]) {
    ui.group(|ui| {
        ui.label(format!("📋 {}", title));
        for instruction in instructions {
            ui.label(*instruction);
        }
    });
}

/// Dibuja un grupo de configuración con un título
pub fn config_group(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.group(|ui| {
        ui.label(format!("⚙️ {}", title));
        ui.add_space(10.0);
        content(ui);
    });
}

/// Dibuja una sección colapsable con scroll
pub fn collapsible_scroll(
    ui: &mut egui::Ui, 
    title: &str, 
    max_height: f32,
    content: impl FnOnce(&mut egui::Ui)
) {
    ui.collapsing(title, |ui| {
        egui::ScrollArea::vertical().max_height(max_height).show(ui, |ui| {
            content(ui);
        });
    });
}

/// Dibuja estadísticas en un grupo
pub fn stats_group(ui: &mut egui::Ui, title: &str, stats: &[(&str, String)]) {
    ui.group(|ui| {
        ui.heading(title);
        ui.add_space(10.0);
        for (label, value) in stats {
            ui.label(format!("{}: {}", label, value));
        }
    });
}
