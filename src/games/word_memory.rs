use super::*;
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

pub struct WordMemoryGame {
    #[allow(dead_code)]
    config: GameConfig,
    state: MemoryState,
    start_time: Option<Instant>,
    words_to_remember: Vec<String>,
    current_word_index: usize,
    user_input: String,
    finished: bool,
    word_display_time: Duration,
    last_word_time: Option<Instant>,
    focus_input: bool,
    matched_words: Vec<String>,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum MemoryState {
    Instructions,
    ShowingWords,
    Recall,
}

impl WordMemoryGame {
    pub fn new(config: GameConfig) -> Self {
        let words = Self::generate_words(&config);
        let word_time = match config.difficulty {
            Difficulty::Easy => Duration::from_millis(3000),
            Difficulty::Medium => Duration::from_millis(2000),
            Difficulty::Hard => Duration::from_millis(1500),
        };
        
        Self {
            config,
            state: MemoryState::Instructions,
            start_time: None,
            words_to_remember: words,
            current_word_index: 0,
            user_input: String::new(),
            finished: false,
            word_display_time: word_time,
            last_word_time: None,
            focus_input: false,
            matched_words: Vec::new(),
            should_go_to_menu: false,
        }
    }

    fn generate_words(config: &GameConfig) -> Vec<String> {
        let word_pools = match config.difficulty {
            Difficulty::Easy => vec![
                "casa", "perro", "sol", "mesa", "libro", "agua", "fuego", "árbol",
                "flor", "cielo", "mar", "montaña", "río", "piedra", "luz", "noche"
            ],
            Difficulty::Medium => vec![
                "computadora", "teléfono", "automóvil", "biblioteca", "hospital", "universidad",
                "restaurante", "supermercado", "farmacia", "aeropuerto", "estación", "oficina"
            ],
            Difficulty::Hard => vec![
                "epistemología", "neuroplasticidad", "fenomenología", "hermenéutica", "paradigma",
                "metamorfosis", "episódico", "cronológico", "metodología", "taxonomía"
            ],
        };
        
        let word_count = match config.difficulty {
            Difficulty::Easy => 8,
            Difficulty::Medium => 12,
            Difficulty::Hard => 16,
        };
        
        let mut words: Vec<String> = word_pools.iter().map(|s| s.to_string()).collect();
        words.shuffle(&mut rand::thread_rng());
        words.truncate(word_count);
        words
    }

    fn calculate_accuracy(&self) -> f32 {
        let user_words: Vec<String> = self.user_input
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        
        let original_words: Vec<String> = self.words_to_remember
            .iter()
            .map(|s| s.to_lowercase())
            .collect();
        
        let mut correct = 0;
        for word in &user_words {
            if original_words.contains(word) {
                correct += 1;
            }
        }
        
        if self.words_to_remember.is_empty() { 
            0.0 
        } else { 
            correct as f32 / self.words_to_remember.len() as f32 
        }
    }
}

impl Game for WordMemoryGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }
        
        match self.state {
            MemoryState::Instructions => {
                ui.horizontal(|ui| {
                    if ui.button("← Menú").clicked() {
                        self.should_go_to_menu = true;
                    }
                });
                ui.separator();
                ui.add_space(10.0);
                
                ui.heading("Juego de Memoria de Palabras");
                ui.separator();
                
                ui.group(|ui| {
                    ui.label("📋 Instrucciones:");
                    ui.label("1. Se mostrarán palabras una por una");
                    ui.label("2. Memoriza todas las palabras que veas");
                    ui.label("3. Después escribe todas las palabras que recuerdes");
                });
                
                ui.add_space(20.0);
                
                ui.group(|ui| {
                    ui.label("⚙️ Configuración:");
                    ui.add_space(10.0);
                    ui.label(format!("Palabras a memorizar: {}", self.words_to_remember.len()));
                    ui.label(format!("Tiempo por palabra: {:.1}s", self.word_display_time.as_secs_f32()));
                });
                
                ui.add_space(20.0);
                
                if ui.button("Comenzar").clicked() {
                    self.state = MemoryState::ShowingWords;
                    self.start_time = Some(Instant::now());
                    self.last_word_time = Some(Instant::now());
                }
            }
            
            MemoryState::ShowingWords => {
                if let Some(word_start) = self.last_word_time {
                    if word_start.elapsed() >= self.word_display_time {
                        self.current_word_index += 1;
                        if self.current_word_index >= self.words_to_remember.len() {
                            self.state = MemoryState::Recall;
                            self.start_time = Some(Instant::now());
                            self.focus_input = true;
                            return;
                        }
                        self.last_word_time = Some(Instant::now());
                    }
                }
                
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    
                    ui.label(format!("Palabra {} de {}", 
                        self.current_word_index + 1, 
                        self.words_to_remember.len()));
                    
                    ui.add_space(20.0);
                    
                    if self.current_word_index < self.words_to_remember.len() {
                        ui.label(egui::RichText::new(&self.words_to_remember[self.current_word_index])
                            .size(36.0)
                            .strong());
                    }
                });
            }
            
            MemoryState::Recall => {
                ui.horizontal(|ui| {
                    if ui.button("← Menú").clicked() {
                        self.should_go_to_menu = true;
                    }
                });
                ui.separator();
                ui.add_space(10.0);
                
                ui.heading("Escribe las palabras que recuerdas:");
                ui.separator();
                
                ui.label("Escribe todas las palabras separadas por espacios:");
                
                let text_edit = egui::TextEdit::multiline(&mut self.user_input)
                    .desired_rows(8)
                    .desired_width(f32::INFINITY)
                    .hint_text("palabra1 palabra2 palabra3...");
                let response = ui.add(text_edit);
                
                // Autofocus
                if self.focus_input {
                    response.request_focus();
                    self.focus_input = false;
                }
                
                ui.add_space(10.0);
                
                // Mostrar cuántas palabras ha escrito el usuario
                let user_word_count = self.user_input.split_whitespace().count();
                ui.label(format!("Palabras escritas: {} / {}", user_word_count, self.words_to_remember.len()));
                
                ui.add_space(10.0);
                
                if button_with_enter(ui, "Terminar") {
                    // Calcular palabras correctas para el resumen
                    let user_words: Vec<String> = self.user_input
                        .split_whitespace()
                        .map(|s| s.to_lowercase())
                        .collect();
                    
                    let original_words: Vec<String> = self.words_to_remember
                        .iter()
                        .map(|s| s.to_lowercase())
                        .collect();
                    
                    self.matched_words = user_words.iter()
                        .filter(|w| original_words.contains(w))
                        .cloned()
                        .collect();
                    
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
        
        let accuracy = self.calculate_accuracy();
        let words_correct = self.matched_words.len();
        
        Some(GameResult {
            game_type: crate::GameType::WordMemory,
            score: accuracy * 100.0,
            details: GameDetails::WordMemory {
                words_correct,
                original_words: self.words_to_remember.clone(),
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, MemoryState::ShowingWords) && !self.finished && !self.should_go_to_menu
    }
}