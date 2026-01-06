use super::*;
use std::time::Instant;

pub struct TextComprehensionGame {
    #[allow(dead_code)]
    config: GameConfig,
    state: ComprehensionState,
    start_time: Option<Instant>,
    text: String,
    questions: Vec<Question>,
    current_question: usize,
    answers: Vec<String>,
    finished: bool,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum ComprehensionState {
    Instructions,
    Reading,
    Questions,
}

#[derive(Debug, Clone)]
struct Question {
    question: String,
    options: Vec<String>,
    correct_answer: usize,
}

impl TextComprehensionGame {
    pub fn new(config: GameConfig) -> Self {
        let (text, questions) = Self::generate_content(&config);
        
        Self {
            config,
            state: ComprehensionState::Instructions,
            start_time: None,
            text,
            questions,
            current_question: 0,
            answers: Vec::new(),
            finished: false,
            should_go_to_menu: false,
        }
    }

    fn generate_content(config: &GameConfig) -> (String, Vec<Question>) {
        let content = match config.difficulty {
            Difficulty::Easy => (
                "El sol es una estrella muy importante para la vida en la Tierra. Nos da luz y calor todos los días. Sin el sol, no podríamos vivir. Las plantas necesitan la luz del sol para crecer y producir oxígeno que respiramos.".to_string(),
                vec![
                    Question {
                        question: "¿Qué es el sol?".to_string(),
                        options: vec!["Un planeta".to_string(), "Una estrella".to_string(), "Una luna".to_string()],
                        correct_answer: 1,
                    },
                    Question {
                        question: "¿Qué necesitan las plantas del sol?".to_string(),
                        options: vec!["Agua".to_string(), "Luz".to_string(), "Tierra".to_string()],
                        correct_answer: 1,
                    }
                ]
            ),
            Difficulty::Medium => (
                "La inteligencia artificial es una tecnología que permite a las máquinas realizar tareas que normalmente requieren inteligencia humana. Incluye el aprendizaje automático, donde los sistemas pueden mejorar su rendimiento a través de la experiencia sin ser programados explícitamente.".to_string(),
                vec![
                    Question {
                        question: "¿Qué permite la inteligencia artificial?".to_string(),
                        options: vec![
                            "Solo calcular números".to_string(), 
                            "Realizar tareas que requieren inteligencia humana".to_string(), 
                            "Reemplazar completamente a los humanos".to_string()
                        ],
                        correct_answer: 1,
                    },
                    Question {
                        question: "¿Cómo mejoran los sistemas de aprendizaje automático?".to_string(),
                        options: vec![
                            "A través de la experiencia".to_string(), 
                            "Solo con programación explícita".to_string(), 
                            "No pueden mejorar".to_string()
                        ],
                        correct_answer: 0,
                    }
                ]
            ),
            Difficulty::Hard => (
                "La neuroplasticidad se refiere a la capacidad del sistema nervioso para cambiar su estructura y función en respuesta a la experiencia. Este fenómeno permite la adaptación, el aprendizaje y la recuperación tras lesiones cerebrales, desafiando la antigua creencia de que el cerebro adulto era inmutable.".to_string(),
                vec![
                    Question {
                        question: "¿Qué es la neuroplasticidad?".to_string(),
                        options: vec![
                            "La rigidez del cerebro".to_string(),
                            "La capacidad del sistema nervioso para cambiar".to_string(),
                            "Una enfermedad cerebral".to_string()
                        ],
                        correct_answer: 1,
                    },
                    Question {
                        question: "¿Qué creencia antigua desafía la neuroplasticidad?".to_string(),
                        options: vec![
                            "Que el cerebro puede cambiar".to_string(),
                            "Que el cerebro adulto era inmutable".to_string(),
                            "Que el aprendizaje es imposible".to_string()
                        ],
                        correct_answer: 1,
                    }
                ]
            ),
        };
        content
    }

    fn calculate_score(&self) -> f32 {
        if self.questions.is_empty() {
            return 0.0;
        }
        
        let mut correct = 0;
        for (i, answer_str) in self.answers.iter().enumerate() {
            if i < self.questions.len() {
                if let Ok(answer_idx) = answer_str.parse::<usize>() {
                    if answer_idx == self.questions[i].correct_answer {
                        correct += 1;
                    }
                }
            }
        }
        
        (correct as f32 / self.questions.len() as f32) * 100.0
    }
}

impl Game for TextComprehensionGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }
        
        match self.state {
            ComprehensionState::Instructions => {
                ui.horizontal(|ui| {
                    if ui.button("← Menú").clicked() {
                        self.should_go_to_menu = true;
                    }
                });
                ui.separator();
                ui.add_space(10.0);
                
                ui.heading("Juego de Comprensión de Texto");
                ui.separator();
                
                ui.group(|ui| {
                    ui.label("📋 Instrucciones:");
                    ui.label("1. Lee el texto cuidadosamente");
                    ui.label("2. Responde las preguntas sobre el texto");
                    ui.label("3. Selecciona la respuesta correcta");
                });
                
                ui.add_space(20.0);
                
                ui.group(|ui| {
                    ui.label("⚙️ Configuración:");
                    ui.add_space(10.0);
                    ui.label(format!("Preguntas: {}", self.questions.len()));
                });
                
                ui.add_space(20.0);
                
                if ui.button("Comenzar").clicked() {
                    self.state = ComprehensionState::Reading;
                    self.start_time = Some(Instant::now());
                }
            }
            
            ComprehensionState::Reading => {
                ui.horizontal(|ui| {
                    if ui.button("← Menú").clicked() {
                        self.should_go_to_menu = true;
                    }
                });
                ui.separator();
                ui.add_space(10.0);
                
                ui.heading("Lee el siguiente texto:");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.label(egui::RichText::new(&self.text).size(16.0));
                });
                
                ui.add_space(20.0);
                
                if ui.button("Continuar a las preguntas").clicked() {
                    self.state = ComprehensionState::Questions;
                    self.answers = vec!["".to_string(); self.questions.len()];
                }
            }
            
            ComprehensionState::Questions => {
                ui.horizontal(|ui| {
                    if ui.button("← Menú").clicked() {
                        self.should_go_to_menu = true;
                    }
                });
                ui.separator();
                ui.add_space(10.0);
                
                ui.heading(format!("Pregunta {} de {}", 
                    self.current_question + 1, 
                    self.questions.len()));
                ui.separator();
                
                if self.current_question < self.questions.len() {
                    let question = &self.questions[self.current_question];
                    
                    ui.label(egui::RichText::new(&question.question).size(18.0));
                    ui.add_space(10.0);
                    
                    let mut selected = self.answers[self.current_question].parse::<usize>().unwrap_or(usize::MAX);
                    
                    for (i, option) in question.options.iter().enumerate() {
                        if ui.radio(selected == i, option).clicked() {
                            selected = i;
                            self.answers[self.current_question] = i.to_string();
                        }
                    }
                    
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        if self.current_question > 0 {
                            if ui.button("Anterior").clicked() {
                                self.current_question -= 1;
                            }
                        }
                        
                        if self.current_question < self.questions.len() - 1 {
                            if ui.button("Siguiente").clicked() {
                                self.current_question += 1;
                            }
                        } else {
                            if ui.button("Terminar").clicked() {
                                self.finished = true;
                            }
                        }
                    });
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
        
        let score = self.calculate_score();
        let correct_answers = self.answers.iter().enumerate()
            .filter(|(i, answer_str)| {
                if let Ok(answer_idx) = answer_str.parse::<usize>() {
                    *i < self.questions.len() && answer_idx == self.questions[*i].correct_answer
                } else {
                    false
                }
            }).count();
        
        Some(GameResult {
            game_type: crate::GameType::TextComprehension,
            score,
            details: GameDetails::TextComprehension {
                questions_correct: correct_answers,
                total_questions: self.questions.len(),
            },
            timestamp: std::time::SystemTime::now(),
        })
    }
}