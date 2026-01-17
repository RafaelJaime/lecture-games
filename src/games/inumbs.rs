use super::*;
use egui::RichText;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct INumbsGame {
    config: GameConfig,
    state: INumbsState,
    start_time: Option<Instant>,
    numbers: Vec<String>,
    current_index: usize,
    display_time: Duration,
    total_count: usize,
    user_inputs: Vec<String>,
    finished: bool,
    should_go_to_menu: bool,
    correct_answers: usize,
    overall_start: Option<Instant>,
    focus_input: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum INumbsState {
    Instructions,
    Showing,
    Filling,
}

impl INumbsGame {
    pub fn new(config: GameConfig) -> Self {
        let total_count = config.word_count.max(1);
        Self {
            config: config.clone(),
            state: INumbsState::Instructions,
            start_time: None,
            numbers: Vec::new(),
            current_index: 0,
            display_time: config.duration,
            total_count,
            user_inputs: Vec::new(),
            finished: false,
            should_go_to_menu: false,
            correct_answers: 0,
            overall_start: None,
            focus_input: false,
        }
    }

    fn generate_numbers(count: usize) -> Vec<String> {
        let mut rng = rand::thread_rng();
        let mut v = Vec::with_capacity(count);
        for _ in 0..count {
            // Generate pairs (00-99)
            let n: u8 = rng.gen_range(0..100);
            v.push(format!("{:02}", n));
        }
        v
    }

    fn draw_menu_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("< Menú").clicked() {
                self.should_go_to_menu = true;
                self.state = INumbsState::Instructions;
            }
        });
        ui.separator();
        ui.add_space(10.0);
    }

    fn calculate_result(&mut self) {
        let mut correct = 0usize;
        for (i, expected) in self.numbers.iter().enumerate() {
            if let Some(ans) = self.user_inputs.get(i) {
                if ans.trim() == expected.trim() {
                    correct += 1;
                }
            }
        }
        self.correct_answers = correct;
    }
}

impl Game for INumbsGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu { return; }

        match self.state {
            INumbsState::Instructions => {
                self.draw_menu_button(ui);
                ui.heading("iNumbs - Entrenamiento Numérico");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("📋 Instrucciones:");
                    ui.label("1. Elige la cantidad de números a memorizar");
                    ui.label("2. Elige los segundos de visualización por número");
                    ui.label("3. Activa 'Rellenar casilleros' si quieres completar los números después");
                    ui.label("4. Presiona Comenzar para iniciar el ejercicio");
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Cantidad de números:");
                        let mut cnt = self.total_count as i32;
                        if ui.add(egui::DragValue::new(&mut cnt).clamp_range(1..=200)).changed() {
                            self.total_count = cnt.max(1) as usize;
                            self.config.word_count = self.total_count;
                        }
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Segundos por número:");
                        let mut secs = self.display_time.as_secs() as i32;
                        // Use DragValue so there's no artificial upper bound. Enforce minimum 1s.
                        if ui.add(egui::DragValue::new(&mut secs).speed(1)).changed() {
                            if secs < 1 { secs = 1; }
                            self.display_time = Duration::from_secs(secs as u64);
                            self.config.duration = self.display_time;
                        }
                    });

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Rellenar casilleros:");
                        let mut fill = self.config.fill_boxes;
                        if ui.checkbox(&mut fill, "").changed() {
                            self.config.fill_boxes = fill;
                        }
                    });

                    ui.add_space(10.0);
                    ui.label(format!("Cantidad: {}", self.total_count));
                    ui.label(format!("Segundos por número: {}s", self.display_time.as_secs()));
                    ui.label(format!("Rellenar casilleros: {}", if self.config.fill_boxes { "Sí" } else { "No" }));
                });

                ui.add_space(20.0);
                if ui.button("Comenzar").clicked() {
                    self.numbers = Self::generate_numbers(self.total_count);
                    self.current_index = 0;
                    self.start_time = Some(Instant::now());
                    self.overall_start = Some(Instant::now());
                    self.user_inputs = vec![String::new(); self.total_count];
                    self.state = INumbsState::Showing;
                }
            }

            INumbsState::Showing => {
                self.draw_menu_button(ui);

                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    if elapsed >= self.display_time {
                        // finished showing all numbers at once
                        if self.config.fill_boxes {
                            self.state = INumbsState::Filling;
                            self.focus_input = true;
                        } else {
                            self.calculate_result();
                            self.finished = true;
                        }
                        return;
                    }

                    let remaining = self.display_time - elapsed;
                    let remaining_ms = remaining.as_millis();

                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.heading(format!("Mostrando todos los números ({}/{})", self.total_count, self.total_count));
                        ui.add_space(10.0);
                        ui.label(format!("Tiempo restante: {} ms", remaining_ms));
                        ui.add_space(10.0);

                        // Show all numbers at once, with small separation. Use columns to wrap.
                        let cols = self.total_count.clamp(1, 12); // up to 12 per row
                        for row in 0..self.total_count.div_ceil(cols) {
                            ui.horizontal(|ui| {
                                for col in 0..cols {
                                    let idx = row * cols + col;
                                    if idx >= self.total_count { break; }
                                    if let Some(num) = self.numbers.get(idx) {
                                        ui.add(egui::Label::new(RichText::new(num).size(28.0).color(egui::Color32::from_rgb(20,120,200)).strong()));
                                        ui.add_space(8.0);
                                    }
                                }
                            });
                            ui.add_space(6.0);
                        }

                        ui.add_space(8.0);
                        ui.label("Memoriza los números en el orden en que aparecen");
                    });
                }
            }

            INumbsState::Filling => {
                self.draw_menu_button(ui);
                ui.heading("Rellenar casilleros");
                ui.separator();
                ui.add_space(10.0);

                ui.label("Introduce los números en el orden que los viste:");
                ui.add_space(10.0);

                // display inputs in a grid
                let cols = 6usize;
                for row in 0..self.total_count.div_ceil(cols) {
                    ui.horizontal(|ui| {
                        for col in 0..cols {
                            let idx = row * cols + col;
                            if idx >= self.total_count { break; }
                            let text_edit = egui::TextEdit::singleline(&mut self.user_inputs[idx]).desired_width(60.0);
                            let response = ui.add(text_edit);
                            
                            // Autofocus on first input
                            if self.focus_input && idx == 0 {
                                response.request_focus();
                                self.focus_input = false;
                            }
                        }
                    });
                }

                ui.add_space(20.0);
                if button_with_enter(ui, "Confirmar") {
                    self.calculate_result();
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
        if !self.finished { return None; }

        let time_taken = self.overall_start.map(|s| s.elapsed()).unwrap_or(Duration::from_secs(0));
        let score = if self.total_count == 0 { 0.0 } else { (self.correct_answers as f32 / self.total_count as f32) * 100.0 };

        Some(GameResult {
            game_type: crate::GameType::INumbs,
            score,
            details: GameDetails::INumbs {
                correct: self.correct_answers,
                total: self.total_count,
                time_taken,
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, INumbsState::Showing) && !self.finished && !self.should_go_to_menu
    }
}
