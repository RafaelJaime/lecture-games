//! Prueba de Exhibición - Variante rápida (0.5 segundos)
//!
//! Reglas:
//! - Tiempo de exposición: 0.5 segundos
//! - Cada fallo por un solo dígito incrementa +0.1 puntos
//! - Solo para exhibiciones, no competición oficial

use super::*;
use crate::models::{GameType, HalfDigitSystem, AttemptResult};
use egui::RichText;
use rand::Rng;
use std::time::Instant;

pub struct ExhibicionGame {
    config: GameConfig,
    state: ExhibicionState,

    // Secuencia actual
    current_sequence: String,
    sequence_length: usize,

    // Input del usuario
    user_input: String,
    focus_input: bool,

    // Timing
    start_time: Option<Instant>,

    // Intentos y resultados
    current_attempt: usize,
    attempt_results: Vec<AttemptResult>,
    best_score: usize,
    bonus_tenths: f32,

    // Estado del juego
    finished: bool,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ExhibicionState {
    Instructions,
    Ready,
    Showing,
    Input,
    Feedback,
    FinalResults,
}

impl ExhibicionGame {
    pub fn new(config: GameConfig) -> Self {
        let initial_length = config.element_count;

        Self {
            config,
            state: ExhibicionState::Instructions,
            current_sequence: String::new(),
            sequence_length: initial_length,
            user_input: String::new(),
            focus_input: false,
            start_time: None,
            current_attempt: 1,
            attempt_results: Vec::new(),
            best_score: 0,
            bonus_tenths: 0.0,
            finished: false,
            should_go_to_menu: false,
        }
    }

    fn generate_sequence(length: usize) -> String {
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| char::from_digit(rng.gen_range(0..10), 10).unwrap())
            .collect()
    }

    fn start_attempt(&mut self) {
        self.current_sequence = Self::generate_sequence(self.sequence_length);
        self.user_input.clear();
        self.state = ExhibicionState::Ready;
    }

    fn check_answer(&mut self) {
        let user_answer = self.user_input.trim().to_string();
        let correct = user_answer == self.current_sequence;

        let errors = if correct {
            0
        } else {
            self.count_errors(&self.current_sequence, &user_answer)
        };

        let result = AttemptResult {
            attempt_number: self.current_attempt,
            sequence_length: self.sequence_length,
            user_answer: user_answer.clone(),
            correct,
            errors,
            time_taken: self.start_time.map(|s| s.elapsed()).unwrap_or_default(),
        };

        self.attempt_results.push(result.clone());

        if correct {
            if self.sequence_length > self.best_score {
                self.best_score = self.sequence_length;
            }
            self.sequence_length += 1;
        } else {
            // Exhibición: cada fallo por 1 dígito suma +0.1
            if errors == 1 {
                self.bonus_tenths += HalfDigitSystem::calculate_exhibition_bonus(1);
            }
        }

        self.state = ExhibicionState::Feedback;
    }

    fn count_errors(&self, expected: &str, actual: &str) -> usize {
        let expected_chars: Vec<char> = expected.chars().collect();
        let actual_chars: Vec<char> = actual.chars().collect();

        let max_len = expected_chars.len().max(actual_chars.len());
        let min_len = expected_chars.len().min(actual_chars.len());

        let mut errors = max_len - min_len;

        for i in 0..min_len {
            if expected_chars[i] != actual_chars[i] {
                errors += 1;
            }
        }

        errors
    }

    fn next_attempt(&mut self) {
        self.current_attempt += 1;

        if self.current_attempt > self.config.max_attempts {
            self.state = ExhibicionState::FinalResults;
        } else {
            self.start_attempt();
        }
    }

    fn draw_menu_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("< Menu").clicked() {
                self.should_go_to_menu = true;
            }
        });
        ui.separator();
        ui.add_space(10.0);
    }
}

impl Game for ExhibicionGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }

        match self.state.clone() {
            ExhibicionState::Instructions => {
                self.draw_menu_button(ui);

                ui.heading("Prueba de Exhibicion");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(RichText::new("MODO EXHIBICION - 0.5 segundos!").color(egui::Color32::GOLD));
                    ui.add_space(10.0);
                    ui.label("Instrucciones:");
                    ui.label("1. Se mostrara una secuencia de digitos por solo 0.5 segundos");
                    ui.label("2. Es una variante ultra-rapida para exhibiciones");
                    ui.label("3. Cada fallo por un solo digito suma +0.1 puntos");
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Configuracion:");
                    ui.label(format!("Modo: {}", self.config.mode.name()));
                    ui.label("Tiempo de exposicion: 0.5 segundos");
                    ui.label(format!("Intentos: {}", self.config.max_attempts));
                    ui.label(format!("Longitud inicial: {} digitos", self.config.element_count));
                });

                ui.add_space(20.0);

                if ui.button("Comenzar").clicked() {
                    self.start_attempt();
                }
            }

            ExhibicionState::Ready => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.label(RichText::new("EXHIBICION").size(24.0).color(egui::Color32::GOLD));
                    ui.heading(format!("Intento {} de {}", self.current_attempt, self.config.max_attempts));
                    ui.add_space(20.0);
                    ui.label(format!("Secuencia de {} digitos", self.sequence_length));
                    ui.label(RichText::new("0.5 segundos de exposicion!").color(egui::Color32::YELLOW));
                    ui.add_space(30.0);

                    if ui.button("Mostrar secuencia").clicked() {
                        self.start_time = Some(Instant::now());
                        self.state = ExhibicionState::Showing;
                    }
                });
            }

            ExhibicionState::Showing => {
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    let exposure = ExposureTime::HalfSecond.as_duration();

                    if elapsed >= exposure {
                        self.state = ExhibicionState::Input;
                        self.focus_input = true;
                        self.start_time = Some(Instant::now());
                        return;
                    }

                    let remaining = exposure - elapsed;

                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(
                            RichText::new(format!("{:.2}s", remaining.as_secs_f32()))
                                .size(20.0)
                                .color(egui::Color32::GOLD)
                        );
                        ui.add_space(40.0);

                        ui.label(
                            RichText::new(&self.current_sequence)
                                .size(56.0)
                                .color(egui::Color32::GOLD)
                                .strong()
                                .monospace()
                        );

                        ui.add_space(40.0);
                        ui.label(format!("{} digitos", self.sequence_length));
                    });
                }
            }

            ExhibicionState::Input => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.heading("Escribe la secuencia");
                    ui.add_space(20.0);
                    ui.label(format!("Secuencia de {} digitos", self.sequence_length));
                });

                ui.add_space(20.0);

                let text_edit = egui::TextEdit::singleline(&mut self.user_input)
                    .desired_width(300.0)
                    .font(egui::TextStyle::Heading)
                    .hint_text("Escribe los digitos...");

                let response = ui.add(text_edit);

                if self.focus_input {
                    response.request_focus();
                    self.focus_input = false;
                }

                ui.add_space(10.0);
                ui.label(format!("Digitos escritos: {}", self.user_input.len()));

                ui.add_space(20.0);

                if button_with_enter(ui, "Confirmar") && !self.user_input.is_empty() {
                    self.check_answer();
                }
            }

            ExhibicionState::Feedback => {
                self.draw_menu_button(ui);

                let last_result = self.attempt_results.last().unwrap();

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    if last_result.correct {
                        ui.label(RichText::new("CORRECTO!").size(32.0).color(egui::Color32::GREEN));
                    } else {
                        ui.label(RichText::new("INCORRECTO").size(32.0).color(egui::Color32::RED));
                        if last_result.errors == 1 {
                            ui.label(RichText::new("+0.1 bonus!").color(egui::Color32::GOLD));
                        }
                    }

                    ui.add_space(20.0);
                });

                ui.group(|ui| {
                    ui.label(format!("Secuencia mostrada: {}", self.current_sequence));
                    ui.label(format!("Tu respuesta: {}", last_result.user_answer));
                    if !last_result.correct {
                        ui.label(format!("Errores: {}", last_result.errors));
                    }
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(format!("Mejor marca actual: {} digitos", self.best_score));
                    if self.bonus_tenths > 0.0 {
                        ui.label(format!("Bonus acumulado: +{:.1}", self.bonus_tenths));
                    }
                    ui.label(format!("Intentos restantes: {}", self.config.max_attempts - self.current_attempt));
                });

                ui.add_space(20.0);

                if self.current_attempt >= self.config.max_attempts {
                    if ui.button("Ver resultados finales").clicked() {
                        self.state = ExhibicionState::FinalResults;
                    }
                } else if ui.button("Siguiente intento").clicked() {
                    self.next_attempt();
                }
            }

            ExhibicionState::FinalResults => {
                self.draw_menu_button(ui);

                ui.heading("Resultados de Exhibicion");
                ui.separator();
                ui.add_space(10.0);

                let final_score = self.best_score as f32 + self.bonus_tenths;

                ui.group(|ui| {
                    ui.label(
                        RichText::new(format!("Puntuacion Final: {:.1}", final_score))
                            .size(28.0)
                            .strong()
                            .color(egui::Color32::GOLD)
                    );
                    ui.add_space(10.0);
                    ui.label(format!("Mejor marca: {} digitos", self.best_score));
                    if self.bonus_tenths > 0.0 {
                        ui.label(format!("Bonus por fallos de 1 digito: +{:.1}", self.bonus_tenths));
                    }
                    ui.label("Tiempo de exposicion: 0.5 segundos");
                });

                ui.add_space(10.0);

                ui.collapsing("Historial de intentos", |ui| {
                    for result in &self.attempt_results {
                        ui.horizontal(|ui| {
                            let status = if result.correct { "OK" } else { "X" };
                            let bonus = if !result.correct && result.errors == 1 { " (+0.1)" } else { "" };
                            ui.label(format!(
                                "#{}: {} digitos - {}{}",
                                result.attempt_number,
                                result.sequence_length,
                                status,
                                bonus
                            ));
                        });
                    }
                });

                ui.add_space(20.0);

                if ui.button("Finalizar").clicked() {
                    self.finished = true;
                }
            }
        }
    }

    fn get_state(&self) -> GameState {
        if self.finished {
            GameState::Finished
        } else if self.should_go_to_menu {
            GameState::Aborted
        } else {
            GameState::Playing
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        if !self.finished {
            return None;
        }

        let final_score = self.best_score as f32 + self.bonus_tenths;

        Some(GameResult {
            game_type: GameType::Exhibicion,
            game_mode: self.config.mode,
            score: final_score,
            details: GameDetails::Exhibicion {
                digits_memorized: self.best_score,
                bonus_tenths: self.bonus_tenths,
                attempt_number: self.current_attempt,
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, ExhibicionState::Showing)
    }
}
