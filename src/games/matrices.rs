//! Prueba de Matrices - Memorización visual de casillas blancas y azules
//!
//! Reglas de competición:
//! - 12 matrices por intento
//! - Duración total: 6 minutos
//! - Velocidad de paso automática y progresiva
//! - 2 intentos totales, solo cuenta la mejor marca
//! - 10 puntos por casilla correctamente memorizada

use super::*;
use crate::models::GameType;
use egui::{Color32, RichText, Pos2, Rect, Vec2};
use rand::Rng;
use std::time::Instant;

pub struct MatricesGame {
    config: GameConfig,
    state: MatricesState,

    // Matrices generadas
    matrices: Vec<Matrix>,
    current_matrix_index: usize,

    // Respuestas del usuario
    user_matrices: Vec<Matrix>,

    // Timing
    start_time: Option<Instant>,
    matrix_start_time: Option<Instant>,

    // Intentos
    current_attempt: usize,
    best_score: usize,

    // Estado del juego
    finished: bool,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone)]
struct Matrix {
    cells: Vec<Vec<bool>>, // true = azul, false = blanco
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            cells: vec![vec![false; cols]; rows],
            rows,
            cols,
        }
    }

    fn generate_random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let cells: Vec<Vec<bool>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.gen_bool(0.5)).collect())
            .collect();

        Self { cells, rows, cols }
    }

    fn compare(&self, other: &Matrix) -> usize {
        let mut correct = 0;
        for (r, row) in self.cells.iter().enumerate() {
            for (c, &cell) in row.iter().enumerate() {
                if r < other.rows && c < other.cols && cell == other.cells[r][c] {
                    correct += 1;
                }
            }
        }
        correct
    }

    fn render(&self, ui: &mut egui::Ui, cell_size: f32, interactive: bool) -> Option<(usize, usize)> {
        let mut clicked_cell = None;
        let spacing = 2.0;

        let total_width = self.cols as f32 * (cell_size + spacing);
        let total_height = self.rows as f32 * (cell_size + spacing);

        let (response, painter) = ui.allocate_painter(
            Vec2::new(total_width, total_height),
            egui::Sense::click(),
        );

        let rect = response.rect;

        for row in 0..self.rows {
            for col in 0..self.cols {
                let x = rect.min.x + col as f32 * (cell_size + spacing);
                let y = rect.min.y + row as f32 * (cell_size + spacing);
                let cell_rect = Rect::from_min_size(
                    Pos2::new(x, y),
                    Vec2::new(cell_size, cell_size),
                );

                let color = if self.cells[row][col] {
                    Color32::from_rgb(30, 100, 200) // Azul
                } else {
                    Color32::WHITE
                };

                painter.rect_filled(cell_rect, 2.0, color);
                painter.rect_stroke(cell_rect, 2.0, egui::Stroke::new(1.0, Color32::GRAY));

                // Detectar click en modo interactivo
                if interactive && response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        if cell_rect.contains(pos) {
                            clicked_cell = Some((row, col));
                        }
                    }
                }
            }
        }

        clicked_cell
    }
}

#[derive(Debug, Clone, PartialEq)]
enum MatricesState {
    Instructions,
    Ready,
    ShowingMatrices,
    InputMatrices,
    FinalResults,
}

impl MatricesGame {
    pub fn new(config: GameConfig) -> Self {
        let (rows, cols) = config.matrix_size;
        let count = config.matrix_count;

        // Generar matrices
        let matrices: Vec<Matrix> = (0..count)
            .map(|_| Matrix::generate_random(rows, cols))
            .collect();

        // Crear matrices vacías para respuestas
        let user_matrices: Vec<Matrix> = (0..count)
            .map(|_| Matrix::new(rows, cols))
            .collect();

        Self {
            config,
            state: MatricesState::Instructions,
            matrices,
            current_matrix_index: 0,
            user_matrices,
            start_time: None,
            matrix_start_time: None,
            current_attempt: 1,
            best_score: 0,
            finished: false,
            should_go_to_menu: false,
        }
    }

    fn calculate_score(&self) -> usize {
        let mut total_correct = 0;
        for (i, matrix) in self.matrices.iter().enumerate() {
            if i < self.user_matrices.len() {
                total_correct += matrix.compare(&self.user_matrices[i]);
            }
        }
        total_correct * 10 // 10 puntos por casilla
    }

    fn start_showing(&mut self) {
        self.current_matrix_index = 0;
        self.start_time = Some(Instant::now());
        self.matrix_start_time = Some(Instant::now());
        self.state = MatricesState::ShowingMatrices;
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

    fn get_progressive_time(&self, index: usize) -> std::time::Duration {
        // Velocidad progresiva: empieza lento y acelera
        let base_secs = 3.0f32;
        let factor = 1.0 - (index as f32 * 0.1).min(0.5); // Reduce hasta 50%
        std::time::Duration::from_secs_f32(base_secs * factor)
    }
}

impl Game for MatricesGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }

        match self.state.clone() {
            MatricesState::Instructions => {
                self.draw_menu_button(ui);

                ui.heading("Prueba de Matrices");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Instrucciones:");
                    ui.label("1. Se mostraran matrices de casillas blancas y azules");
                    ui.label("2. Memoriza el patron de cada matriz");
                    ui.label("3. Despues dibujaras todas las matrices de memoria");
                    ui.label("4. Haz clic en las casillas para marcarlas como azules");
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Configuracion:");
                    ui.label(format!("Modo: {}", self.config.mode.name()));
                    ui.label(format!("Matrices: {}", self.config.matrix_count));
                    ui.label(format!("Tamano: {}x{}", self.config.matrix_size.0, self.config.matrix_size.1));
                    ui.label(format!("Intentos: {}", self.config.max_attempts));
                });

                ui.add_space(10.0);

                // Preview de matriz
                ui.group(|ui| {
                    ui.label("Vista previa del tamano:");
                    let preview = Matrix::generate_random(self.config.matrix_size.0, self.config.matrix_size.1);
                    preview.render(ui, 20.0, false);
                });

                ui.add_space(20.0);

                if ui.button("Comenzar").clicked() {
                    self.state = MatricesState::Ready;
                }
            }

            MatricesState::Ready => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(format!("Intento {} de {}", self.current_attempt, self.config.max_attempts));
                    ui.add_space(20.0);
                    ui.label(format!("{} matrices de {}x{}",
                        self.config.matrix_count,
                        self.config.matrix_size.0,
                        self.config.matrix_size.1
                    ));
                    ui.add_space(10.0);
                    ui.label("La velocidad aumentara progresivamente");
                    ui.add_space(30.0);

                    if ui.button("Iniciar visualizacion").clicked() {
                        self.start_showing();
                    }
                });
            }

            MatricesState::ShowingMatrices => {
                if let Some(matrix_start) = self.matrix_start_time {
                    let display_time = self.get_progressive_time(self.current_matrix_index);
                    let elapsed = matrix_start.elapsed();

                    if elapsed >= display_time {
                        // Pasar a la siguiente matriz
                        self.current_matrix_index += 1;
                        if self.current_matrix_index >= self.matrices.len() {
                            self.state = MatricesState::InputMatrices;
                            self.current_matrix_index = 0;
                            return;
                        }
                        self.matrix_start_time = Some(Instant::now());
                        return;
                    }

                    let remaining = display_time - elapsed;

                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        ui.label(format!(
                            "Matriz {} de {}",
                            self.current_matrix_index + 1,
                            self.matrices.len()
                        ));
                        ui.label(format!("Tiempo: {:.1}s", remaining.as_secs_f32()));
                        ui.add_space(20.0);

                        if let Some(matrix) = self.matrices.get(self.current_matrix_index) {
                            matrix.render(ui, 40.0, false);
                        }

                        ui.add_space(20.0);
                        ui.label("Memoriza el patron");
                    });
                }
            }

            MatricesState::InputMatrices => {
                self.draw_menu_button(ui);

                ui.horizontal(|ui| {
                    ui.heading(format!(
                        "Dibuja la matriz {} de {}",
                        self.current_matrix_index + 1,
                        self.matrices.len()
                    ));
                });

                ui.add_space(10.0);
                ui.label("Haz clic en las casillas para marcarlas como azules");
                ui.add_space(20.0);

                // Renderizar matriz interactiva
                if self.current_matrix_index < self.user_matrices.len() {
                    let mut user_matrix = self.user_matrices[self.current_matrix_index].clone();

                    if let Some((row, col)) = user_matrix.render(ui, 40.0, true) {
                        // Toggle la celda
                        user_matrix.cells[row][col] = !user_matrix.cells[row][col];
                        self.user_matrices[self.current_matrix_index] = user_matrix;
                    }
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if self.current_matrix_index > 0 && ui.button("< Anterior").clicked() {
                        self.current_matrix_index -= 1;
                    }

                    if self.current_matrix_index < self.matrices.len() - 1 {
                        if ui.button("Siguiente >").clicked() {
                            self.current_matrix_index += 1;
                        }
                    } else if ui.button("Finalizar").clicked() {
                        // Calcular puntuación
                        let score = self.calculate_score();
                        if score > self.best_score {
                            self.best_score = score;
                        }
                        self.state = MatricesState::FinalResults;
                    }
                });

                // Navegación rápida
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Ir a matriz:");
                    for i in 0..self.matrices.len() {
                        let label = format!("{}", i + 1);
                        if ui.selectable_label(i == self.current_matrix_index, label).clicked() {
                            self.current_matrix_index = i;
                        }
                    }
                });
            }

            MatricesState::FinalResults => {
                self.draw_menu_button(ui);

                ui.heading("Resultados de Matrices");
                ui.separator();
                ui.add_space(10.0);

                let total_cells = self.config.matrix_size.0 * self.config.matrix_size.1 * self.matrices.len();

                ui.group(|ui| {
                    ui.label(RichText::new(format!("Puntuacion: {} puntos", self.best_score)).size(28.0).strong());
                    ui.add_space(10.0);
                    ui.label(format!("Casillas totales: {}", total_cells));
                    ui.label(format!("Casillas correctas: {}", self.best_score / 10));
                    ui.label(format!(
                        "Precision: {:.1}%",
                        (self.best_score as f32 / 10.0) / total_cells as f32 * 100.0
                    ));
                });

                ui.add_space(10.0);

                // Mostrar comparación
                ui.collapsing("Ver comparacion", |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for (i, (original, user)) in self.matrices.iter().zip(self.user_matrices.iter()).enumerate() {
                            ui.horizontal(|ui| {
                                ui.group(|ui| {
                                    ui.label(format!("#{} Original:", i + 1));
                                    original.render(ui, 15.0, false);
                                });
                                ui.group(|ui| {
                                    ui.label("Tu respuesta:");
                                    user.render(ui, 15.0, false);
                                });
                                ui.label(format!(
                                    "{}/{}",
                                    original.compare(user),
                                    self.config.matrix_size.0 * self.config.matrix_size.1
                                ));
                            });
                            ui.separator();
                        }
                    });
                });

                ui.add_space(20.0);

                if self.current_attempt < self.config.max_attempts {
                    if ui.button("Siguiente intento").clicked() {
                        self.current_attempt += 1;
                        // Regenerar matrices
                        let (rows, cols) = self.config.matrix_size;
                        self.matrices = (0..self.config.matrix_count)
                            .map(|_| Matrix::generate_random(rows, cols))
                            .collect();
                        self.user_matrices = (0..self.config.matrix_count)
                            .map(|_| Matrix::new(rows, cols))
                            .collect();
                        self.current_matrix_index = 0;
                        self.state = MatricesState::Ready;
                    }
                }

                if ui.button("Finalizar prueba").clicked() {
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

        let cells_per_matrix = self.config.matrix_size.0 * self.config.matrix_size.1;

        Some(GameResult {
            game_type: GameType::Matrices,
            game_mode: self.config.mode,
            score: self.best_score as f32,
            details: GameDetails::Matrices {
                matrices_correct: self.best_score / 10 / cells_per_matrix,
                total_matrices: self.matrices.len(),
                cells_per_matrix,
                points: self.best_score,
                attempt_number: self.current_attempt,
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, MatricesState::ShowingMatrices)
    }
}
