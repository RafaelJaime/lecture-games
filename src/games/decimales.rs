//! Prueba de Decimales - Memorización de secuencias de dígitos (0-9)
//!
//! Reglas de competición:
//! - Tiempo de exposición: 1 segundo o 4 segundos
//! - 10 intentos totales
//! - Solo cuenta la mejor marca
//! - Sistema de medio dígito aplica desde 14 (1s) o 20 (4s) dígitos

use super::*;
use crate::models::{GameType, HalfDigitSystem, AttemptResult};
use egui::RichText;
use rand::Rng;
use std::time::Instant;

pub struct DecimalesGame {
    config: GameConfig,
    state: DecimalesState,

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
    half_digit_bonus: f32,

    // Estado del juego
    finished: bool,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum DecimalesState {
    Instructions,
    Ready,
    Showing,
    Input,
    Feedback,
    FinalResults,
}

impl DecimalesGame {
    pub fn new(config: GameConfig) -> Self {
        let initial_length = config.element_count;

        Self {
            config,
            state: DecimalesState::Instructions,
            current_sequence: String::new(),
            sequence_length: initial_length,
            user_input: String::new(),
            focus_input: false,
            start_time: None,
            current_attempt: 1,
            attempt_results: Vec::new(),
            best_score: 0,
            half_digit_bonus: 0.0,
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
        self.state = DecimalesState::Ready;
    }

    fn check_answer(&mut self) {
        let user_answer = self.user_input.trim().to_string();
        let correct = user_answer == self.current_sequence;

        // Contar errores (diferencias entre secuencias)
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
            // Actualizar mejor marca
            if self.sequence_length > self.best_score {
                self.best_score = self.sequence_length;
            }
            // Aumentar dificultad para el siguiente intento
            self.sequence_length += 1;
        } else {
            // Calcular bonus de medio dígito si aplica
            if errors == 1 && self.sequence_length == self.best_score + 1 {
                let bonus = HalfDigitSystem::calculate_bonus(
                    self.best_score,
                    self.sequence_length,
                    errors,
                    &self.config.exposure_time,
                );
                if bonus > self.half_digit_bonus {
                    self.half_digit_bonus = bonus;
                }
            }
        }

        self.state = DecimalesState::Feedback;
    }

    fn count_errors(&self, expected: &str, actual: &str) -> usize {
        let expected_chars: Vec<char> = expected.chars().collect();
        let actual_chars: Vec<char> = actual.chars().collect();

        let max_len = expected_chars.len().max(actual_chars.len());
        let min_len = expected_chars.len().min(actual_chars.len());

        let mut errors = max_len - min_len; // Diferencia de longitud

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
            self.state = DecimalesState::FinalResults;
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

impl Game for DecimalesGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }

        match self.state.clone() {
            DecimalesState::Instructions => {
                self.draw_menu_button(ui);

                ui.heading("Prueba de Decimales");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Instrucciones:");
                    ui.label("1. Se mostrara una secuencia de digitos (0-9)");
                    ui.label("2. Memoriza la secuencia durante el tiempo de exposicion");
                    ui.label("3. Escribe la secuencia exacta despues");
                    ui.label("4. Si aciertas, la siguiente secuencia sera mas larga");
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Configuracion:");
                    ui.label(format!("Modo: {}", self.config.mode.name()));
                    ui.label(format!("Tiempo de exposicion: {}", self.config.exposure_time.name()));
                    ui.label(format!("Intentos: {}", self.config.max_attempts));
                    ui.label(format!("Longitud inicial: {} digitos", self.config.element_count));
                });

                ui.add_space(20.0);

                if ui.button("Comenzar").clicked() {
                    self.start_attempt();
                }
            }

            DecimalesState::Ready => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(format!("Intento {} de {}", self.current_attempt, self.config.max_attempts));
                    ui.add_space(20.0);
                    ui.label(format!("Secuencia de {} digitos", self.sequence_length));
                    ui.add_space(30.0);

                    if ui.button("Mostrar secuencia").clicked() {
                        self.start_time = Some(Instant::now());
                        self.state = DecimalesState::Showing;
                    }
                });
            }

            DecimalesState::Showing => {
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    let exposure = self.config.exposure_time.as_duration();

                    if elapsed >= exposure {
                        self.state = DecimalesState::Input;
                        self.focus_input = true;
                        self.start_time = Some(Instant::now());
                        return;
                    }

                    let remaining = exposure - elapsed;

                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(format!("Tiempo: {:.1}s", remaining.as_secs_f32()));
                        ui.add_space(40.0);

                        // Mostrar la secuencia con estilo grande
                        ui.label(
                            RichText::new(&self.current_sequence)
                                .size(48.0)
                                .color(egui::Color32::from_rgb(50, 100, 200))
                                .strong()
                                .monospace()
                        );

                        ui.add_space(40.0);
                        ui.label(format!("{} digitos", self.sequence_length));
                    });
                }
            }

            DecimalesState::Input => {
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

            DecimalesState::Feedback => {
                self.draw_menu_button(ui);

                let last_result = self.attempt_results.last().unwrap();

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    if last_result.correct {
                        ui.label(RichText::new("CORRECTO!").size(32.0).color(egui::Color32::GREEN));
                    } else {
                        ui.label(RichText::new("INCORRECTO").size(32.0).color(egui::Color32::RED));
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
                    if self.half_digit_bonus > 0.0 {
                        ui.label(format!("Bonus medio digito: +{}", self.half_digit_bonus));
                    }
                    ui.label(format!("Intentos restantes: {}", self.config.max_attempts - self.current_attempt));
                });

                ui.add_space(20.0);

                if self.current_attempt >= self.config.max_attempts {
                    if ui.button("Ver resultados finales").clicked() {
                        self.state = DecimalesState::FinalResults;
                    }
                } else if ui.button("Siguiente intento").clicked() {
                    self.next_attempt();
                }
            }

            DecimalesState::FinalResults => {
                self.draw_menu_button(ui);

                ui.heading("Resultados Finales");
                ui.separator();
                ui.add_space(10.0);

                let final_score = self.best_score as f32 + self.half_digit_bonus;

                ui.group(|ui| {
                    ui.label(RichText::new(format!("Puntuacion Final: {}", final_score)).size(28.0).strong());
                    ui.add_space(10.0);
                    ui.label(format!("Mejor marca: {} digitos", self.best_score));
                    if self.half_digit_bonus > 0.0 {
                        ui.label(format!("Bonus medio digito: +{}", self.half_digit_bonus));
                    }
                    ui.label(format!("Tiempo de exposicion: {}", self.config.exposure_time.name()));
                });

                ui.add_space(10.0);

                ui.collapsing("Historial de intentos", |ui| {
                    for result in &self.attempt_results {
                        ui.horizontal(|ui| {
                            let status = if result.correct { "OK" } else { "X" };
                            let detail = if result.correct {
                                "correcto".to_string()
                            } else {
                                format!("{} errores", result.errors)
                            };
                            ui.label(format!(
                                "#{}: {} digitos - {} ({})",
                                result.attempt_number,
                                result.sequence_length,
                                status,
                                detail
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

        let final_score = self.best_score as f32 + self.half_digit_bonus;

        Some(GameResult {
            game_type: GameType::Decimales,
            game_mode: self.config.mode,
            score: final_score,
            details: GameDetails::Decimales {
                digits_memorized: self.best_score,
                exposure_time: self.config.exposure_time,
                half_digit_bonus: self.half_digit_bonus,
                attempt_number: self.current_attempt,
                is_best: true,
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, DecimalesState::Showing)
    }
}
