use super::*;
use egui::RichText;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct ReadingSpeedGame {
    config: GameConfig,
    state: NumberState,
    training_mode: bool,
    start_time: Option<Instant>,
    number_to_remember: String,
    user_input: String,
    display_time: Duration,
    digit_count: usize,
    current_round: usize,
    total_rounds: usize,
    correct_answers: usize,
    round_results: Vec<RoundResult>,
    finished: bool,
    should_go_to_menu: bool,
    base_digit_count: usize,
}

#[derive(Debug, Clone)]
struct RoundResult {
    number: String,
    user_answer: String,
    correct: bool,
    accuracy: f32,
}

#[derive(Debug, Clone, PartialEq)]
enum NumberState {
    Instructions,
    ShowingNumber,
    Writing,
}

impl ReadingSpeedGame {
    pub fn new(config: GameConfig) -> Self {
        let (digit_count, display_time) = Self::get_config_params(&config);
        let number = Self::generate_number(digit_count);
        Self {
            config,
            state: NumberState::Instructions,
            training_mode: false,
            start_time: None,
            number_to_remember: number,
            user_input: String::new(),
            display_time,
            digit_count,
            current_round: 1,
            total_rounds: 10,
            correct_answers: 0,
            round_results: Vec::new(),
            finished: false,
            should_go_to_menu: false,
            base_digit_count: digit_count,
        }
    }

    fn get_config_params(config: &GameConfig) -> (usize, Duration) {
        let digit_count = match config.difficulty {
            Difficulty::Easy => rand::thread_rng().gen_range(1..=6),
            Difficulty::Medium => rand::thread_rng().gen_range(7..=10),
            Difficulty::Hard => rand::thread_rng().gen_range(11..=20),
        };
        
        let display_time = config.duration;
        
        (digit_count, display_time)
    }

    fn generate_number(digit_count: usize) -> String {
        let mut rng = rand::thread_rng();
        let mut number = String::new();
        
        if digit_count > 1 {
            number.push_str(&rng.gen_range(1..=9).to_string());
        } else {
            number.push_str(&rng.gen_range(0..=9).to_string());
        }
        
        for _ in 1..digit_count {
            number.push_str(&rng.gen_range(0..=9).to_string());
        }
        
        number
    }

    fn get_difficulty_name(difficulty: &Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Easy => "Fácil (1-6 dígitos)",
            Difficulty::Medium => "Medio (7-10 dígitos)", 
            Difficulty::Hard => "Difícil (11-20 dígitos)",
        }
    }

    fn calculate_digit_count_for_round(&self, round: usize) -> usize {
        if !self.training_mode {
            // Modo normal: usar el rango según dificultad
            match self.config.difficulty {
                Difficulty::Easy => rand::thread_rng().gen_range(1..=6),
                Difficulty::Medium => rand::thread_rng().gen_range(7..=10),
                Difficulty::Hard => rand::thread_rng().gen_range(11..=20),
            }
        } else {
            // Modo training: progresión gradual
            let base_count = self.base_digit_count;
            
            match self.config.difficulty {
                Difficulty::Easy => {
                    // Aumenta 1 dígito cada ronda
                    base_count + (round - 1)
                },
                Difficulty::Medium => {
                    // Aumenta 1 dígito cada 2 rondas
                    base_count + ((round - 1) / 2)
                },
                Difficulty::Hard => {
                    // Aumenta 1 dígito cada 3 rondas (comenzando desde la ronda 10)
                    if round >= 10 {
                        base_count + ((round - 10) / 3)
                    } else {
                        base_count
                    }
                },
            }
        }
    }

    fn calculate_round_accuracy(&self) -> f32 {
        let original = self.number_to_remember.trim();
        let user = self.user_input.trim();
        
        if original.is_empty() {
            return 0.0;
        }
        
        if original == user {
            100.0
        } else {
            let original_chars: Vec<char> = original.chars().collect();
            let user_chars: Vec<char> = user.chars().collect();
            
            let mut correct = 0;
            let max_len = original_chars.len().max(user_chars.len());
            
            for i in 0..original_chars.len().min(user_chars.len()) {
                if original_chars[i] == user_chars[i] {
                    correct += 1;
                }
            }
            
            if max_len == 0 { 0.0 } else { (correct as f32 / max_len as f32) * 100.0 }
        }
    }

    fn calculate_overall_accuracy(&self) -> f32 {
        if self.round_results.is_empty() {
            return 0.0;
        }
        
        let total_accuracy: f32 = self.round_results.iter().map(|r| r.accuracy).sum();
        total_accuracy / self.round_results.len() as f32
    }

    fn next_round(&mut self) {
        let accuracy = self.calculate_round_accuracy();
        let is_correct = self.number_to_remember.trim() == self.user_input.trim();
        
        if is_correct {
            self.correct_answers += 1;
        }
        
        self.round_results.push(RoundResult {
            number: self.number_to_remember.clone(),
            user_answer: self.user_input.clone(),
            correct: is_correct,
            accuracy,
        });
        
        self.user_input.clear();
        
        if self.current_round >= self.total_rounds {
            self.finished = true;
        } else {
            self.current_round += 1;
            
            // Calcular el número de dígitos para la siguiente ronda
            let next_digit_count = self.calculate_digit_count_for_round(self.current_round);
            self.digit_count = next_digit_count;
            self.number_to_remember = Self::generate_number(next_digit_count);
            self.state = NumberState::ShowingNumber;
            self.start_time = Some(Instant::now());
        }
    }

    fn draw_menu_button(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("← Menú").clicked() {
                self.should_go_to_menu = true;
                self.state = NumberState::Instructions;
            }
        });
        ui.separator();
        ui.add_space(10.0);
    }
}

impl Game for ReadingSpeedGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        // Verificar si se debe ir al menú
        if self.should_go_to_menu {
            return;
        }
        
        match self.state {
            NumberState::Instructions => {
                self.draw_menu_button(ui);
                
                ui.heading("Juego de Memoria Numérica");
                ui.separator();
                ui.add_space(10.0);
                
                ui.group(|ui| {
                    ui.label("📋 Instrucciones:");
                    ui.label("1. Configura el tiempo de visualización, dificultad y número de rondas");
                    ui.label("2. En cada ronda aparecerá un número durante el tiempo configurado");
                    ui.label("3. Después tendrás que escribir el número de memoria");
                    ui.label("4. Completa todas las rondas para obtener tu puntuación final");
                });
                
                ui.add_space(20.0);
                
                ui.group(|ui| {
                    ui.label("⚙️ Configuración:");
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Tiempo de visualización (milisegundos):");
                        let mut duration_millis = self.config.duration.as_millis() as u64;
                        if ui.add(egui::Slider::new(&mut duration_millis, 500..=3000).text("ms")).changed() {
                            self.config.duration = Duration::from_millis(duration_millis);
                            self.display_time = self.config.duration;
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Número de rondas:");
                        ui.radio_value(&mut self.total_rounds, 10, "10");
                        ui.radio_value(&mut self.total_rounds, 20, "20");
                        ui.radio_value(&mut self.total_rounds, 30, "30");
                    });
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Training:");
                        ui.checkbox(&mut self.training_mode, "");
                    });
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Dificultad:");
                        if ui.radio_value(&mut self.config.difficulty, Difficulty::Easy, "Fácil").changed() {
                            let (digit_count, _) = Self::get_config_params(&self.config);
                            self.base_digit_count = digit_count;
                            self.digit_count = digit_count;
                            self.number_to_remember = Self::generate_number(digit_count);
                        }
                        if ui.radio_value(&mut self.config.difficulty, Difficulty::Medium, "Medio").changed() {
                            let (digit_count, _) = Self::get_config_params(&self.config);
                            self.base_digit_count = digit_count;
                            self.digit_count = digit_count;
                            self.number_to_remember = Self::generate_number(digit_count);
                        }
                        if ui.radio_value(&mut self.config.difficulty, Difficulty::Hard, "Difícil").changed() {
                            let (digit_count, _) = Self::get_config_params(&self.config);
                            self.base_digit_count = digit_count;
                            self.digit_count = digit_count;
                            self.number_to_remember = Self::generate_number(digit_count);
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.label(format!("Tiempo de visualización: {} ms", self.config.duration.as_millis()));
                    ui.label(format!("Número de rondas: {}", self.total_rounds));
                    ui.label(format!("Training: {}", if self.training_mode { "Activado" } else { "Desactivado" }));
                    ui.label(format!("Dificultad: {}", Self::get_difficulty_name(&self.config.difficulty)));
                    ui.label(format!("Rango de dígitos: {}", match self.config.difficulty {
                        Difficulty::Easy => "1-6",
                        Difficulty::Medium => "7-10",
                        Difficulty::Hard => "11-20",
                    }));
                });
                
                ui.add_space(20.0);
                
                if ui.button("Comenzar").clicked() {
                    let (digit_count, display_time) = Self::get_config_params(&self.config);
                    self.base_digit_count = digit_count;
                    self.digit_count = digit_count;
                    self.display_time = display_time;
                    self.number_to_remember = Self::generate_number(digit_count);
                    self.state = NumberState::ShowingNumber;
                    self.start_time = Some(Instant::now());
                }
            }
            
            NumberState::ShowingNumber => {
                self.draw_menu_button(ui);
                
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    if elapsed >= self.display_time {
                        self.state = NumberState::Writing;
                        self.start_time = Some(Instant::now());
                        return;
                    }
                    
                    let remaining = self.display_time - elapsed;
                    let remaining_millis = remaining.as_millis();
                    
                    ui.vertical_centered(|ui| {
                        ui.add_space(30.0);
                        ui.heading(format!("Ronda {} de {}", self.current_round, self.total_rounds));
                        ui.add_space(20.0);
                        ui.label(format!("Tiempo restante: {}ms", remaining_millis));
                        ui.add_space(30.0);
                        
                        ui.label(RichText::new(&self.number_to_remember)
                            .size(48.0)
                            .color(egui::Color32::from_rgb(50, 50, 200))
                            .strong());
                        
                        ui.add_space(20.0);
                        ui.label(format!("Memoriza este número de {} dígitos", self.digit_count));
                        
                        if self.training_mode {
                            ui.label(format!("Training - Ronda {}: {} dígitos", 
                                self.current_round, self.digit_count));
                        }
                        
                        if !self.round_results.is_empty() {
                            ui.add_space(20.0);
                            ui.label(format!("Correctas hasta ahora: {} / {}", 
                                self.correct_answers, 
                                self.round_results.len()));
                        }
                    });
                }
            }
            
            NumberState::Writing => {
                self.draw_menu_button(ui);
                
                ui.vertical_centered(|ui| {
                    ui.heading(format!("Ronda {} de {}", self.current_round, self.total_rounds));
                });
                ui.separator();
                ui.add_space(20.0);
                
                ui.label(format!("El número tenía {} dígitos:", self.digit_count));
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Tu respuesta:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.user_input)
                            .desired_width(200.0)
                            .font(egui::TextStyle::Heading)
                    );
                });
                
                ui.add_space(20.0);
                
                if !self.user_input.is_empty() {
                    ui.label(format!("Dígitos ingresados: {}/{}", self.user_input.len(), self.digit_count));
                }
                
                if !self.round_results.is_empty() {
                    ui.add_space(10.0);
                    ui.label(format!("Progreso: {} / {} rondas completadas", 
                        self.round_results.len(), 
                        self.total_rounds));
                    ui.label(format!("Correctas: {} / {}", 
                        self.correct_answers, 
                        self.round_results.len()));
                }
                
                ui.add_space(20.0);
                
                if ui.button("Confirmar respuesta").clicked() {
                    self.next_round();
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
        
        let overall_accuracy = self.calculate_overall_accuracy();
        
        Some(GameResult {
            game_type: crate::GameType::ReadingSpeed,
            score: overall_accuracy,
            details: GameDetails::ReadingSpeed {
                words_correct: self.correct_answers,
                total_words: self.total_rounds,
                time_taken: self.start_time?.elapsed(),
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, NumberState::ShowingNumber) && !self.finished && !self.should_go_to_menu
    }
}