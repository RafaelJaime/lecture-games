use super::*;
use egui::RichText;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

pub struct ReadingSpeedGame {
    config: GameConfig,
    state: ReadingState,
    start_time: Option<Instant>,
    text_to_read: String,
    user_input: String,
    time_remaining: Duration,
    finished: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ReadingState {
    Instructions,
    Reading,
    Writing,
}

impl ReadingSpeedGame {
    pub fn new(config: GameConfig) -> Self {
        let text = Self::generate_text(&config);
        Self {
            config,
            state: ReadingState::Instructions,
            start_time: None,
            text_to_read: text,
            user_input: String::new(),
            time_remaining: Duration::from_secs(60),
            finished: false,
        }
    }

    fn generate_text(config: &GameConfig) -> String {
        let texts = match config.difficulty {
            Difficulty::Easy => vec![
                "El sol brillaba en el cielo azul. Los pájaros cantaban alegremente en los árboles verdes.",
                "La casa era grande y hermosa. Tenía un jardín lleno de flores rojas y amarillas.",
                "El gato dormía sobre la mesa. Era un gato negro con ojos verdes.",
            ],
            Difficulty::Medium => vec![
                "La tecnología ha transformado completamente nuestra forma de comunicarnos. Las redes sociales permiten conectar con personas de todo el mundo instantáneamente.",
                "El cambio climático es uno de los mayores retos de nuestro tiempo. Las temperaturas globales continúan aumentando.",
                "La inteligencia artificial está revolucionando múltiples sectores de la economía.",
            ],
            Difficulty::Hard => vec![
                "La epistemología contemporánea ha experimentado una transformación paradigmática significativa.",
                "Los mecanismos neuroplásticos involucrados en la consolidación de la memoria a largo plazo requieren activación sincronizada.",
                "La complejidad algorítmica de los sistemas de procesamiento de lenguaje natural ha evolucionado exponencialmente.",
            ],
        };
        texts.choose(&mut rand::thread_rng()).unwrap().to_string()
    }

    fn get_difficulty_name(difficulty: &Difficulty) -> &'static str {
        match difficulty {
            Difficulty::Easy => "Fácil",
            Difficulty::Medium => "Medio", 
            Difficulty::Hard => "Difícil",
        }
    }

    fn calculate_accuracy(&self) -> f32 {
        let original_words: Vec<&str> = self.text_to_read.split_whitespace().collect();
        let user_words: Vec<&str> = self.user_input.split_whitespace().collect();
        
        let mut correct = 0;
        let total = original_words.len().max(user_words.len());
        
        for i in 0..original_words.len().min(user_words.len()) {
            if original_words[i].to_lowercase() == user_words[i].to_lowercase() {
                correct += 1;
            }
        }
        
        if total == 0 { 0.0 } else { correct as f32 / total as f32 }
    }
}

impl Game for ReadingSpeedGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        match self.state {
            ReadingState::Instructions => {
                ui.heading("Juego de Lectura Rápida");
                ui.separator();
                ui.add_space(10.0);
                
                // Instructions section
                ui.group(|ui| {
                    ui.label("📋 Instrucciones:");
                    ui.label("1. Configura la duración y dificultad del juego");
                    ui.label("2. Lee el texto que aparecerá en el tiempo configurado");
                    ui.label("3. Después tendrás que escribirlo de memoria");
                    ui.label("4. Trata de ser lo más preciso posible");
                });
                
                ui.add_space(20.0);
                
                // Configuration section
                ui.group(|ui| {
                    ui.label("⚙️ Configuración:");
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Tiempo de lectura (segundos):");
                        let mut duration_secs = self.config.duration.as_secs();
                        if ui.add(egui::Slider::new(&mut duration_secs, 10..=120).text("s")).changed() {
                            self.config.duration = Duration::from_secs(duration_secs);
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.label("Dificultad:");
                        ui.radio_value(&mut self.config.difficulty, Difficulty::Easy, "Fácil");
                        ui.radio_value(&mut self.config.difficulty, Difficulty::Medium, "Medio");
                        ui.radio_value(&mut self.config.difficulty, Difficulty::Hard, "Difícil");
                    });
                    
                    ui.add_space(10.0);
                    ui.label(format!("Duración seleccionada: {} segundos", self.config.duration.as_secs()));
                    ui.label(format!("Dificultad: {}", Self::get_difficulty_name(&self.config.difficulty)));
                });
                
                ui.add_space(20.0);
                
                if ui.button("Comenzar").clicked() {
                    // Regenerate text with new configuration
                    self.text_to_read = Self::generate_text(&self.config);
                    self.state = ReadingState::Reading;
                    self.start_time = Some(Instant::now());
                    self.time_remaining = self.config.duration;
                }
            }
            
            ReadingState::Reading => {
                if let Some(start) = self.start_time {
                    let elapsed = start.elapsed();
                    if elapsed >= self.time_remaining {
                        self.state = ReadingState::Writing;
                        self.start_time = Some(Instant::now());
                        return;
                    }
                    
                    let remaining = self.time_remaining - elapsed;
                    let remaining_secs = remaining.as_secs();
                    
                    ui.heading(format!("Tiempo restante: {}s", remaining_secs));
                    ui.separator();
                    
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label(RichText::new(&self.text_to_read).size(18.0));
                    });
                }
            }
            
            ReadingState::Writing => {
                ui.heading("Escribe el texto que leíste:");
                ui.separator();
                
                ui.label("Intenta recordar el texto exacto:");
                
                ui.add(
                    egui::TextEdit::multiline(&mut self.user_input)
                        .desired_rows(10)
                        .desired_width(f32::INFINITY)
                );
                
                ui.add_space(10.0);
                
                if ui.button("Terminar").clicked() {
                    self.finished = true;
                }
            }
        }
    }

    fn get_state(&self) -> GameState {
        if self.finished {
            GameState::Finished
        } else {
            GameState::Playing
        }
    }

    fn get_result(&self) -> Option<GameResult> {
        if !self.finished {
            return None;
        }
        
        let accuracy = self.calculate_accuracy();
        let original_words: Vec<&str> = self.text_to_read.split_whitespace().collect();
        let words_correct = (accuracy * original_words.len() as f32) as usize;
        
        Some(GameResult {
            game_type: crate::GameType::ReadingSpeed,
            score: accuracy * 100.0,
            details: GameDetails::ReadingSpeed {
                words_correct,
                total_words: original_words.len(),
                time_taken: self.start_time?.elapsed(),
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, ReadingState::Reading) && !self.finished
    }
}