//! Prueba de Binarios - Memorización de secuencias binarias (0 y 1)
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

pub struct BinariosGame {
    config: GameConfig,
    game_type: GameType,
    state: BinariosState,

    // Secuencia actual
    current_sequence: String,
    sequence_length: usize,

    // Input del usuario
    user_input: String,
    focus_input: bool,

    // Fast mode: celdas individuales
    fast_mode_cells: Vec<String>,
    fast_mode_current_cell: usize,

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
enum BinariosState {
    Instructions,
    Ready,
    Showing,
    Input,
    Feedback,
    FinalResults,
}

impl BinariosGame {
    pub fn new(config: GameConfig) -> Self {
        let initial_length = config.element_count;
        // Determinar el game_type basado en el tiempo de exposición
        let game_type = match config.exposure_time {
            ExposureTime::FourSeconds => GameType::Binarios4s,
            _ => GameType::Binarios1s,
        };

        Self {
            config,
            game_type,
            state: BinariosState::Instructions,
            current_sequence: String::new(),
            sequence_length: initial_length,
            user_input: String::new(),
            focus_input: false,
            fast_mode_cells: Vec::new(),
            fast_mode_current_cell: 0,
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
            .map(|_| if rng.gen_bool(0.5) { '1' } else { '0' })
            .collect()
    }

    fn start_attempt(&mut self) {
        self.current_sequence = Self::generate_sequence(self.sequence_length);
        self.user_input.clear();
        // Inicializar celdas para fast mode
        self.fast_mode_cells = vec![String::new(); self.sequence_length];
        self.fast_mode_current_cell = 0;
        self.state = BinariosState::Ready;
    }

    fn check_answer(&mut self) {
        // En fast mode, construir la respuesta desde las celdas
        let user_answer = if self.config.fast_mode {
            self.fast_mode_cells.join("")
        } else {
            self.user_input.trim().to_string()
        };
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
            // Sistema de medio dígito
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

        self.state = BinariosState::Feedback;
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
            self.state = BinariosState::FinalResults;
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

    fn render_fast_mode_input(&mut self, ui: &mut egui::Ui) {
        let cols = self.config.digit_columns;
        let sequence_chars: Vec<char> = self.current_sequence.chars().collect();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = self.config.row_spacing;

            let mut cell_idx = 0;
            for chunk in sequence_chars.chunks(cols) {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = self.config.col_spacing;

                    for (i, expected_char) in chunk.iter().enumerate() {
                        let idx = cell_idx + i;
                        if idx >= self.fast_mode_cells.len() {
                            break;
                        }

                        let cell_value = &self.fast_mode_cells[idx];
                        let is_filled = !cell_value.is_empty();
                        let is_correct = cell_value == &expected_char.to_string();

                        // Determinar color de fondo
                        let bg_color = if is_filled {
                            if is_correct {
                                egui::Color32::from_rgb(50, 150, 50) // Verde
                            } else {
                                egui::Color32::from_rgb(200, 50, 50) // Rojo
                            }
                        } else if idx == self.fast_mode_current_cell {
                            egui::Color32::from_rgb(100, 100, 200) // Azul (celda actual)
                        } else {
                            egui::Color32::from_gray(60)
                        };

                        let frame = egui::Frame::none()
                            .fill(bg_color)
                            .inner_margin(egui::Margin::same(8.0))
                            .rounding(4.0);

                        frame.show(ui, |ui| {
                            let display_text = if is_filled {
                                cell_value.clone()
                            } else {
                                "_".to_string()
                            };

                            ui.label(
                                RichText::new(display_text)
                                    .size(28.0)
                                    .color(egui::Color32::WHITE)
                                    .monospace()
                            );
                        });
                    }
                });
                cell_idx += chunk.len();
            }
        });

        ui.add_space(20.0);

        // Capturar entrada de teclado (solo 0 y 1 para binarios)
        let digit_pressed: Option<char> = ui.input(|i| {
            if i.key_pressed(egui::Key::Num0) {
                Some('0')
            } else if i.key_pressed(egui::Key::Num1) {
                Some('1')
            } else {
                None
            }
        });

        let backspace_pressed = ui.input(|i| i.key_pressed(egui::Key::Backspace));

        if let Some(digit) = digit_pressed {
            if self.fast_mode_current_cell < self.fast_mode_cells.len() {
                self.fast_mode_cells[self.fast_mode_current_cell] = digit.to_string();
                self.fast_mode_current_cell += 1;

                // Auto-confirmar si todas las celdas están llenas
                if self.fast_mode_current_cell >= self.fast_mode_cells.len() {
                    self.check_answer();
                }
            }
        }

        if backspace_pressed && self.fast_mode_current_cell > 0 {
            self.fast_mode_current_cell -= 1;
            self.fast_mode_cells[self.fast_mode_current_cell].clear();
        }

        ui.label(format!(
            "Digitos: {}/{}",
            self.fast_mode_cells.iter().filter(|c| !c.is_empty()).count(),
            self.sequence_length
        ));

        ui.add_space(10.0);
        ui.label("Escribe 0 o 1 con el teclado");
        ui.label("Backspace para borrar");
    }

    fn render_sequence_display(&self, ui: &mut egui::Ui) {
        let cols = self.config.digit_columns;
        let chars: Vec<char> = self.current_sequence.chars().collect();

        // Si la secuencia es corta, mostrarla en una sola línea
        if chars.len() <= cols {
            ui.horizontal(|ui| {
                for c in &chars {
                    let color = if *c == '1' {
                        egui::Color32::from_rgb(50, 150, 50) // Verde para 1
                    } else {
                        egui::Color32::from_rgb(200, 50, 50) // Rojo para 0
                    };
                    ui.label(
                        RichText::new(c.to_string())
                            .size(42.0)
                            .color(color)
                            .strong()
                            .monospace()
                    );
                }
            });
        } else {
            // Dividir en filas según las columnas configuradas
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = self.config.row_spacing;

                for chunk in chars.chunks(cols) {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = self.config.col_spacing;
                        for c in chunk {
                            let color = if *c == '1' {
                                egui::Color32::from_rgb(50, 150, 50) // Verde para 1
                            } else {
                                egui::Color32::from_rgb(200, 50, 50) // Rojo para 0
                            };
                            ui.label(
                                RichText::new(c.to_string())
                                    .size(36.0)
                                    .color(color)
                                    .strong()
                                    .monospace()
                            );
                        }
                    });
                }
            });
        }
    }
}

impl Game for BinariosGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }

        match self.state.clone() {
            BinariosState::Instructions => {
                self.draw_menu_button(ui);

                ui.heading("Prueba de Binarios");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Instrucciones:");
                    ui.label("1. Se mostrara una secuencia de digitos binarios (0 y 1)");
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

            BinariosState::Ready => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(format!("Intento {} de {}", self.current_attempt, self.config.max_attempts));
                    ui.add_space(20.0);
                    ui.label(format!("Secuencia de {} digitos binarios", self.sequence_length));
                    ui.add_space(30.0);

                    if ui.button("Mostrar secuencia").clicked() {
                        self.start_time = Some(Instant::now());
                        self.state = BinariosState::Showing;
                    }
                });
            }

            BinariosState::Showing => {
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    let exposure = self.config.exposure_time.as_duration();

                    if elapsed >= exposure {
                        self.state = BinariosState::Input;
                        self.focus_input = true;
                        self.start_time = Some(Instant::now());
                        return;
                    }

                    let remaining = exposure - elapsed;

                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.label(format!("Tiempo: {:.1}s", remaining.as_secs_f32()));
                        ui.add_space(40.0);

                        // Mostrar la secuencia organizada en columnas
                        self.render_sequence_display(ui);

                        ui.add_space(40.0);
                        ui.label(format!("{} digitos", self.sequence_length));
                    });
                }
            }

            BinariosState::Input => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.heading("Escribe la secuencia");
                    ui.add_space(20.0);
                    ui.label(format!("Secuencia de {} digitos binarios", self.sequence_length));
                    ui.label("(Solo 0 y 1)");
                });

                ui.add_space(20.0);

                if self.config.fast_mode {
                    // Fast mode: celdas individuales con corrección automática
                    self.render_fast_mode_input(ui);
                } else {
                    // Modo estándar: campo de texto único
                    let text_edit = egui::TextEdit::singleline(&mut self.user_input)
                        .desired_width(300.0)
                        .font(egui::TextStyle::Heading)
                        .hint_text("Escribe 0s y 1s...");

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
            }

            BinariosState::Feedback => {
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
                        self.state = BinariosState::FinalResults;
                    }
                } else if ui.button("Siguiente intento").clicked() {
                    self.next_attempt();
                }
            }

            BinariosState::FinalResults => {
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
            game_type: self.game_type,
            game_mode: self.config.mode,
            score: final_score,
            details: GameDetails::Binarios {
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
        matches!(self.state, BinariosState::Showing)
    }
}
