//! Vista de Speed Test y datos del competidor

use eframe::egui;
use crate::models::{GameMode, AppState, CompetitorData};
use crate::controllers::AppController;
use std::time::{Duration, Instant};
use rand::Rng;

/// Estado del Speed Test
pub struct SpeedTestState {
    pub phase: SpeedTestPhase,
    pub sequence: Vec<u8>,
    pub user_input: String,
    pub start_time: Option<Instant>,
    pub results: Vec<f64>, // Tiempos en segundos
    pub current_round: usize,
    pub total_rounds: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpeedTestPhase {
    Instructions,
    Ready,
    Showing,
    Input,
    Results,
}

impl Default for SpeedTestState {
    fn default() -> Self {
        Self {
            phase: SpeedTestPhase::Instructions,
            sequence: Vec::new(),
            user_input: String::new(),
            start_time: None,
            results: Vec::new(),
            current_round: 0,
            total_rounds: 3,
        }
    }
}

impl SpeedTestState {
    pub fn generate_sequence(&mut self) {
        let mut rng = rand::thread_rng();
        self.sequence = (0..4).map(|_| rng.gen_range(0..10)).collect();
    }

    pub fn get_average_time(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.results.iter().sum::<f64>() / self.results.len() as f64
        }
    }
}

/// Renderiza la pantalla de Speed Test
pub fn render_speed_test(
    ui: &mut egui::Ui,
    controller: &mut AppController,
    mode: GameMode,
    state: &mut SpeedTestState,
) {
    ui.vertical_centered(|ui| {
        ui.heading("Speed Test");
        ui.add_space(10.0);
        ui.label(format!("Modo: {}", mode.name()));
        ui.add_space(20.0);
    });

    match state.phase {
        SpeedTestPhase::Instructions => {
            render_instructions(ui, state);
        }
        SpeedTestPhase::Ready => {
            render_ready(ui, state);
        }
        SpeedTestPhase::Showing => {
            render_showing(ui, state);
        }
        SpeedTestPhase::Input => {
            render_input(ui, state);
        }
        SpeedTestPhase::Results => {
            render_results(ui, controller, mode, state);
        }
    }

    // Botón para saltar (solo en instrucciones)
    if state.phase == SpeedTestPhase::Instructions {
        ui.add_space(30.0);
        ui.horizontal(|ui| {
            if ui.button("< Volver").clicked() {
                controller.go_to_mode_selection();
            }
            if ui.button("Omitir Speed Test").clicked() {
                controller.set_state(AppState::CompetitorInfo(mode));
            }
        });
    }
}

fn render_instructions(ui: &mut egui::Ui, state: &mut SpeedTestState) {
    ui.group(|ui| {
        ui.label("El Speed Test mide tu velocidad de reaccion y memoria inmediata.");
        ui.add_space(10.0);
        ui.label("Instrucciones:");
        ui.label("1. Se mostrara una secuencia de 4 digitos");
        ui.label("2. Escribe los digitos lo mas rapido posible");
        ui.label("3. Se realizaran 3 rondas");
        ui.label("4. Se calculara tu tiempo promedio");
    });

    ui.add_space(20.0);

    if ui.button("Comenzar").clicked() {
        state.current_round = 1;
        state.results.clear();
        state.phase = SpeedTestPhase::Ready;
    }
}

fn render_ready(ui: &mut egui::Ui, state: &mut SpeedTestState) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Ronda {}/{}", state.current_round, state.total_rounds));
        ui.add_space(20.0);
        ui.heading("Preparate...");
        ui.add_space(20.0);

        if ui.button("Mostrar secuencia").clicked() {
            state.generate_sequence();
            state.user_input.clear();
            state.start_time = Some(Instant::now());
            state.phase = SpeedTestPhase::Showing;
        }
    });
}

fn render_showing(ui: &mut egui::Ui, state: &mut SpeedTestState) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Ronda {}/{}", state.current_round, state.total_rounds));
        ui.add_space(30.0);

        // Mostrar secuencia
        let sequence_str: String = state.sequence.iter().map(|d| d.to_string()).collect();
        ui.heading(egui::RichText::new(&sequence_str).size(60.0).monospace());

        ui.add_space(30.0);
        ui.label("Memoriza estos digitos");

        // Cambiar a input después de 1 segundo
        if let Some(start) = state.start_time {
            if start.elapsed() >= Duration::from_secs(1) {
                state.phase = SpeedTestPhase::Input;
            }
        }
    });
}

fn render_input(ui: &mut egui::Ui, state: &mut SpeedTestState) {
    ui.vertical_centered(|ui| {
        ui.label(format!("Ronda {}/{}", state.current_round, state.total_rounds));
        ui.add_space(20.0);
        ui.heading("Escribe la secuencia!");
        ui.add_space(20.0);

        let response = ui.add(
            egui::TextEdit::singleline(&mut state.user_input)
                .font(egui::TextStyle::Heading)
                .desired_width(150.0)
                .horizontal_align(egui::Align::Center)
        );

        response.request_focus();

        ui.add_space(20.0);

        // Verificar si la entrada es correcta
        if state.user_input.len() >= 4 {
            if let Some(start) = state.start_time {
                let elapsed = start.elapsed().as_secs_f64();
                state.results.push(elapsed);

                if state.current_round < state.total_rounds {
                    state.current_round += 1;
                    state.phase = SpeedTestPhase::Ready;
                } else {
                    state.phase = SpeedTestPhase::Results;
                }
            }
        }
    });
}

fn render_results(
    ui: &mut egui::Ui,
    controller: &mut AppController,
    mode: GameMode,
    state: &mut SpeedTestState,
) {
    ui.vertical_centered(|ui| {
        ui.heading("Resultados del Speed Test");
        ui.add_space(20.0);
    });

    ui.group(|ui| {
        for (i, time) in state.results.iter().enumerate() {
            ui.label(format!("Ronda {}: {:.3}s", i + 1, time));
        }
        ui.separator();
        ui.label(format!("Tiempo promedio: {:.3}s", state.get_average_time()));
    });

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        if ui.button("Reintentar").clicked() {
            *state = SpeedTestState::default();
        }

        if ui.button("Continuar").clicked() {
            controller.set_state(AppState::CompetitorInfo(mode));
        }
    });
}

/// Estado para datos del competidor
#[derive(Default)]
pub struct CompetitorInfoState {
    pub data: CompetitorData,
}

/// Renderiza la pantalla de datos del competidor
pub fn render_competitor_info(
    ui: &mut egui::Ui,
    controller: &mut AppController,
    mode: GameMode,
    state: &mut CompetitorInfoState,
) {
    ui.vertical_centered(|ui| {
        ui.heading("Datos del Competidor");
        ui.add_space(10.0);
        ui.label(format!("Modo: {}", mode.name()));
        ui.add_space(20.0);
    });

    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label("Numero de asiento:");
            ui.text_edit_singleline(&mut state.data.seat_number);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Titulo (opcional):");
            ui.text_edit_singleline(&mut state.data.title);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Nombre:");
            ui.text_edit_singleline(&mut state.data.name);
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.label("Clave de seguridad:");
            ui.add(egui::TextEdit::singleline(&mut state.data.security_key).password(true));
        });
    });

    ui.add_space(30.0);

    ui.horizontal(|ui| {
        if ui.button("< Volver").clicked() {
            controller.set_state(AppState::SpeedTest(mode));
        }

        let can_continue = !state.data.name.is_empty();

        if ui.add_enabled(can_continue, egui::Button::new("Continuar")).clicked() {
            controller.set_state(AppState::GameSelection(mode));
        }
    });

    if state.data.name.is_empty() {
        ui.add_space(10.0);
        ui.colored_label(egui::Color32::YELLOW, "El nombre es obligatorio");
    }
}
