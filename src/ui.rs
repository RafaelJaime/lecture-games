use crate::*;
use crate::games::*;
use crate::utils::time_format::SystemTimeFormat;

impl SuperlecturaApp {
    pub fn show_game_selection(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading("Juegos de Superlectura");
            ui.label("Basados en el libro de Tony Buzan");
            ui.add_space(30.0);
        });
        ui.horizontal(|ui| {
            if ui.button("📊 Ver Historial").clicked() {
                self.state = AppState::History;
            }
        });
        ui.add_space(20.0);
        ui.separator();
        ui.add_space(20.0);
        let games = [
            GameType::ReadingSpeed,
            GameType::WordMemory,
            GameType::TextComprehension,
        ];
        for game_type in &games {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(game_type.name());
                        ui.label(game_type.description());
                        
                        let stats = self.storage.get_stats_for_game(game_type);
                        if stats.total_games > 0 {
                            ui.label(format!("Partidas jugadas: {}", stats.total_games));
                            ui.label(format!("Mejor puntuación: {:.1}", stats.best_score));
                        } else {
                            ui.label("Sin partidas jugadas");
                        }
                    });
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Jugar").clicked() {
                            // Get or create default config for this game type
                            let config = self.game_configs.get(game_type).cloned()
                                .unwrap_or_else(GameConfig::default);
                            
                            // Create the game instance directly
                            let game: Box<dyn Game> = match game_type {
                                GameType::ReadingSpeed => Box::new(crate::games::reading_speed::ReadingSpeedGame::new(config)),
                                GameType::WordMemory => Box::new(crate::games::word_memory::WordMemoryGame::new(config)),
                                GameType::TextComprehension => Box::new(crate::games::text_comprehension::TextComprehensionGame::new(config)),
                            };
                            
                            self.current_game = Some(game);
                            self.state = AppState::Playing(game_type.clone());
                        }
                    });
                });
            });
            ui.add_space(10.0);
        }
    }

    pub fn show_game_config(&mut self, ui: &mut egui::Ui, game_type: GameType) {
        ui.heading(format!("Configuración: {}", game_type.name()));
        ui.separator();
        ui.add_space(20.0);
        
        let config = self.game_configs.get_mut(&game_type).unwrap();
        
        ui.horizontal(|ui| {
            ui.label("Duración (segundos):");
            let mut duration_secs = config.duration.as_secs();
            if ui.add(egui::Slider::new(&mut duration_secs, 10..=300)).changed() {
                config.duration = std::time::Duration::from_secs(duration_secs);
            }
        });
        
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Dificultad:");
            ui.radio_value(&mut config.difficulty, Difficulty::Easy, "Fácil");
            ui.radio_value(&mut config.difficulty, Difficulty::Medium, "Medio");
            ui.radio_value(&mut config.difficulty, Difficulty::Hard, "Difícil");
        });
        
        ui.add_space(30.0);
        ui.horizontal(|ui| {
            if ui.button("← Volver").clicked() {
                self.state = AppState::GameSelection;
            }
            
            if ui.button("Comenzar Juego").clicked() {
                self.storage.save_config(game_type.clone(), config.clone());
                
                let game: Box<dyn Game> = match game_type {
                    GameType::ReadingSpeed => Box::new(crate::games::reading_speed::ReadingSpeedGame::new(config.clone())),
                    GameType::WordMemory => Box::new(crate::games::word_memory::WordMemoryGame::new(config.clone())),
                    GameType::TextComprehension => Box::new(crate::games::text_comprehension::TextComprehensionGame::new(config.clone())),
                };
                
                self.current_game = Some(game);
                self.state = AppState::Playing(game_type);
            }
        });
    }

    pub fn show_game_play(&mut self, ui: &mut egui::Ui, _game_type: GameType, ctx: &egui::Context) {
        if let Some(game) = &mut self.current_game {
            game.update(ui, ctx);
            
            match game.get_state() {
                GameState::Playing => {},
                GameState::Finished => {
                    if let Some(result) = game.get_result() {
                        self.storage.save_result(result.clone());
                        self.current_result = Some(result);
                        self.state = AppState::Results;
                        self.current_game = None;
                    }
                }
                GameState::Aborted => {
                    self.state = AppState::GameSelection;
                    self.current_game = None;
                }
            }
        }
    }

    pub fn show_results(&mut self, ui: &mut egui::Ui) {
        if let Some(result) = &self.current_result {
            
            ui.vertical_centered(|ui| {
                ui.heading("¡Juego Completado!");
                ui.add_space(20.0);
            });
            
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Juego: {}", result.game_type.name()));
                        ui.label(format!("Puntuación: {:.1}", result.score));
                        ui.label(format!("Fecha: {}", result.timestamp.format_dm_yhm()));
                    });
                });
            });
            
            ui.add_space(20.0);
            match &result.details {
                GameDetails::ReadingSpeed { words_correct, total_words, .. } => {
                    ui.label(format!("Palabras correctas: {} de {}", words_correct, total_words));
                }
                GameDetails::WordMemory { words_correct, original_words, .. } => {
                    ui.label(format!("Palabras recordadas: {} de {}", words_correct, original_words.len()));
                }
                GameDetails::TextComprehension { questions_correct, total_questions, .. } => {
                    ui.label(format!("Respuestas correctas: {} de {}", questions_correct, total_questions));
                }
            }
            
            ui.add_space(30.0);
        }
        
        // Move the buttons outside the borrow to fix the borrow checker issue
        ui.horizontal(|ui| {
            if ui.button("Jugar de nuevo").clicked() {
                if let Some(result) = &self.current_result {
                    self.state = AppState::GameConfig(result.game_type.clone());
                    self.current_result = None;
                }
            }
            
            if ui.button("Menú principal").clicked() {
                self.state = AppState::GameSelection;
                self.current_result = None;
            }
        });
    }

    pub fn show_history(&mut self, ui: &mut egui::Ui) {
        ui.heading("Historial de Partidas");
        ui.separator();
        ui.add_space(10.0);
        
        if ui.button("← Volver al Menú").clicked() {
            self.state = AppState::GameSelection;
            return;
        }
        
        ui.add_space(10.0);
        
        let results = self.storage.get_all_results();
        
        if results.is_empty() {
            ui.label("No hay partidas guardadas");
            return;
        }
        
        // Mostrar estadísticas generales
        ui.collapsing("📊 Resumen General", |ui| {
            let total_games = results.len();
            let total_score: f32 = results.iter().map(|r| r.score).sum();
            let avg_score = if total_games > 0 { total_score / total_games as f32 } else { 0.0 };
            
            ui.label(format!("Total de partidas: {}", total_games));
            ui.label(format!("Puntuación promedio: {:.1}", avg_score));
            
            // Estadísticas por juego
            for game_type in [GameType::ReadingSpeed, GameType::WordMemory, GameType::TextComprehension] {
                let game_results: Vec<_> = results.iter()
                    .filter(|r| r.game_type == game_type)
                    .collect();
                
                if !game_results.is_empty() {
                    ui.separator();
                    ui.strong(game_type.name());
                    ui.label(format!("  Partidas: {}", game_results.len()));
                    
                    let best_score = game_results.iter().map(|r| r.score).fold(0.0f32, |a, b| a.max(b));
                    let avg_score = game_results.iter().map(|r| r.score).sum::<f32>() / game_results.len() as f32;
                    
                    ui.label(format!("  Mejor puntuación: {:.1}", best_score));
                    ui.label(format!("  Puntuación promedio: {:.1}", avg_score));
                }
            }
        });
        
        ui.add_space(20.0);
        
        // Lista de resultados
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                let mut filtered_results = results.clone();
                
                // Ordenar por fecha más reciente
                filtered_results.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
                
                for (i, result) in filtered_results.iter().enumerate() {
                    if i > 0 {
                        ui.separator();
                    }
                    
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.strong(result.game_type.name());
                                ui.label(format!("📅 {}", result.timestamp.format_dm_yhm()));
                                ui.label(format!("🏆 Puntuación: {:.1}", result.score));
                            });
                            
                            ui.vertical(|ui| {
                                match &result.details {
                                    GameDetails::ReadingSpeed { words_correct, total_words, .. } => {
                                        ui.label(format!("Palabras: {}/{}", words_correct, total_words));
                                    }
                                    GameDetails::WordMemory { words_correct, original_words, .. } => {
                                        ui.label(format!("Memorizado: {}/{}", words_correct, original_words.len()));
                                    }
                                    GameDetails::TextComprehension { questions_correct, total_questions, .. } => {
                                        ui.label(format!("Respuestas: {}/{}", questions_correct, total_questions));
                                    }
                                }
                            });
                        });
                    });
                }
            });
        
        ui.add_space(20.0);
        
        // Opciones adicionales
        if ui.button("🗑️ Limpiar Historial").clicked() {
            self.storage.clear_all_results();
        }
    }
}