//! Prueba de Figuras de Colores - Memorización de figuras geométricas con color
//!
//! Reglas de competición:
//! - 15 figuras por intento
//! - Duración total: 3 minutos
//! - Velocidad inicial elegida, luego aumenta automáticamente
//! - 3 intentos totales, solo cuenta la mejor marca
//! - Mínimo 10 figuras para puntuar en velocidades rápidas

use super::*;
use crate::models::GameType;
use egui::{Color32, RichText, Pos2, Vec2};
use rand::Rng;
use std::f32::consts::PI;
use std::time::Instant;

/// Las 8 formas posibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Square,
    Rectangle,
    Triangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Star,
}

impl Shape {
    fn all() -> Vec<Shape> {
        vec![
            Shape::Circle, Shape::Square, Shape::Rectangle, Shape::Triangle,
            Shape::Pentagon, Shape::Hexagon, Shape::Heptagon, Shape::Star,
        ]
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle => "Circulo",
            Shape::Square => "Cuadrado",
            Shape::Rectangle => "Rectangulo",
            Shape::Triangle => "Triangulo",
            Shape::Pentagon => "Pentagono",
            Shape::Hexagon => "Hexagono",
            Shape::Heptagon => "Heptagono",
            Shape::Star => "Estrella",
        }
    }

    fn random() -> Shape {
        let shapes = Self::all();
        shapes[rand::thread_rng().gen_range(0..shapes.len())]
    }
}

/// Los 10 colores posibles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeColor {
    White,
    Black,
    Brown,
    Red,
    Green,
    Yellow,
    Orange,
    Blue,
    Gray,
    Violet,
}

impl ShapeColor {
    fn all() -> Vec<ShapeColor> {
        vec![
            ShapeColor::White, ShapeColor::Black, ShapeColor::Brown,
            ShapeColor::Red, ShapeColor::Green, ShapeColor::Yellow,
            ShapeColor::Orange, ShapeColor::Blue, ShapeColor::Gray,
            ShapeColor::Violet,
        ]
    }

    fn name(&self) -> &str {
        match self {
            ShapeColor::White => "Blanco",
            ShapeColor::Black => "Negro",
            ShapeColor::Brown => "Marron",
            ShapeColor::Red => "Rojo",
            ShapeColor::Green => "Verde",
            ShapeColor::Yellow => "Amarillo",
            ShapeColor::Orange => "Naranja",
            ShapeColor::Blue => "Azul",
            ShapeColor::Gray => "Gris",
            ShapeColor::Violet => "Violeta",
        }
    }

    fn to_color32(&self) -> Color32 {
        match self {
            ShapeColor::White => Color32::WHITE,
            ShapeColor::Black => Color32::BLACK,
            ShapeColor::Brown => Color32::from_rgb(139, 69, 19),
            ShapeColor::Red => Color32::from_rgb(220, 20, 60),
            ShapeColor::Green => Color32::from_rgb(34, 139, 34),
            ShapeColor::Yellow => Color32::from_rgb(255, 215, 0),
            ShapeColor::Orange => Color32::from_rgb(255, 140, 0),
            ShapeColor::Blue => Color32::from_rgb(30, 100, 200),
            ShapeColor::Gray => Color32::from_rgb(128, 128, 128),
            ShapeColor::Violet => Color32::from_rgb(148, 0, 211),
        }
    }

    fn random() -> ShapeColor {
        let colors = Self::all();
        colors[rand::thread_rng().gen_range(0..colors.len())]
    }
}

/// Una figura con forma y color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Figure {
    shape: Shape,
    color: ShapeColor,
}

impl Figure {
    fn random() -> Self {
        Self {
            shape: Shape::random(),
            color: ShapeColor::random(),
        }
    }

    fn render(&self, ui: &mut egui::Ui, size: f32) {
        let (response, painter) = ui.allocate_painter(
            Vec2::new(size, size),
            egui::Sense::hover(),
        );

        let rect = response.rect;
        let center = rect.center();
        let radius = size / 2.0 - 4.0;
        let color = self.color.to_color32();

        // Borde para colores claros
        let stroke_color = if matches!(self.color, ShapeColor::White | ShapeColor::Yellow) {
            Color32::DARK_GRAY
        } else {
            Color32::TRANSPARENT
        };
        let stroke = egui::Stroke::new(2.0, stroke_color);

        match self.shape {
            Shape::Circle => {
                painter.circle_filled(center, radius, color);
                painter.circle_stroke(center, radius, stroke);
            }
            Shape::Square => {
                let half = radius * 0.8;
                let rect = egui::Rect::from_center_size(center, Vec2::splat(half * 2.0));
                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(rect, 0.0, stroke);
            }
            Shape::Rectangle => {
                let rect = egui::Rect::from_center_size(
                    center,
                    Vec2::new(radius * 1.6, radius),
                );
                painter.rect_filled(rect, 0.0, color);
                painter.rect_stroke(rect, 0.0, stroke);
            }
            Shape::Triangle => {
                let points = Self::regular_polygon_points(center, radius, 3, -PI / 2.0);
                painter.add(egui::Shape::convex_polygon(points, color, stroke));
            }
            Shape::Pentagon => {
                let points = Self::regular_polygon_points(center, radius, 5, -PI / 2.0);
                painter.add(egui::Shape::convex_polygon(points, color, stroke));
            }
            Shape::Hexagon => {
                let points = Self::regular_polygon_points(center, radius, 6, 0.0);
                painter.add(egui::Shape::convex_polygon(points, color, stroke));
            }
            Shape::Heptagon => {
                let points = Self::regular_polygon_points(center, radius, 7, -PI / 2.0);
                painter.add(egui::Shape::convex_polygon(points, color, stroke));
            }
            Shape::Star => {
                let points = Self::star_points(center, radius, radius * 0.5, 5);
                painter.add(egui::Shape::convex_polygon(points, color, stroke));
            }
        }
    }

    fn regular_polygon_points(center: Pos2, radius: f32, sides: usize, rotation: f32) -> Vec<Pos2> {
        (0..sides)
            .map(|i| {
                let angle = rotation + 2.0 * PI * i as f32 / sides as f32;
                Pos2::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
            })
            .collect()
    }

    fn star_points(center: Pos2, outer_r: f32, inner_r: f32, points: usize) -> Vec<Pos2> {
        let mut result = Vec::with_capacity(points * 2);
        for i in 0..(points * 2) {
            let angle = -PI / 2.0 + PI * i as f32 / points as f32;
            let r = if i % 2 == 0 { outer_r } else { inner_r };
            result.push(Pos2::new(
                center.x + r * angle.cos(),
                center.y + r * angle.sin(),
            ));
        }
        result
    }
}

pub struct FigurasColoresGame {
    config: GameConfig,
    state: FigurasState,

    // Figuras generadas
    figures: Vec<Figure>,
    current_figure_index: usize,

    // Respuestas del usuario
    user_answers: Vec<Option<Figure>>,
    current_answer_shape: Option<Shape>,
    current_answer_color: Option<ShapeColor>,

    // Timing
    start_time: Option<Instant>,
    figure_start_time: Option<Instant>,
    is_blank_phase: bool, // Fase de tiempo en blanco entre figuras

    // Intentos
    current_attempt: usize,
    best_score: usize,

    // Estado del juego
    finished: bool,
    should_go_to_menu: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum FigurasState {
    Instructions,
    Ready,
    ShowingFigures,
    InputFigures,
    FinalResults,
}

impl FigurasColoresGame {
    pub fn new(config: GameConfig) -> Self {
        let count = config.element_count;
        let figures: Vec<Figure> = (0..count).map(|_| Figure::random()).collect();

        Self {
            config,
            state: FigurasState::Instructions,
            figures,
            current_figure_index: 0,
            user_answers: vec![None; count],
            current_answer_shape: None,
            current_answer_color: None,
            start_time: None,
            figure_start_time: None,
            is_blank_phase: false,
            current_attempt: 1,
            best_score: 0,
            finished: false,
            should_go_to_menu: false,
        }
    }

    fn calculate_score(&self) -> usize {
        let mut correct = 0;
        for (i, figure) in self.figures.iter().enumerate() {
            if let Some(answer) = &self.user_answers[i] {
                if answer.shape == figure.shape && answer.color == figure.color {
                    correct += 1;
                }
            }
        }
        correct
    }

    fn start_showing(&mut self) {
        self.current_figure_index = 0;
        self.start_time = Some(Instant::now());
        self.figure_start_time = Some(Instant::now());
        self.is_blank_phase = false;
        self.state = FigurasState::ShowingFigures;
    }

    fn get_display_time(&self, index: usize) -> std::time::Duration {
        if self.config.constant_time {
            // Tiempo constante para todas las figuras
            std::time::Duration::from_millis(self.config.initial_speed_ms)
        } else {
            // La velocidad aumenta progresivamente
            let base_ms = self.config.initial_speed_ms as f32;
            let factor = 1.0 - (index as f32 * 0.03).min(0.4); // Reduce hasta 40%
            std::time::Duration::from_millis((base_ms * factor) as u64)
        }
    }

    fn get_blank_time(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.config.figure_blank_time_ms)
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

impl Game for FigurasColoresGame {
    fn update(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        if self.should_go_to_menu {
            return;
        }

        match self.state.clone() {
            FigurasState::Instructions => {
                self.draw_menu_button(ui);

                ui.heading("Prueba de Figuras de Colores");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Instrucciones:");
                    ui.label("1. Se mostraran figuras geometricas con colores");
                    ui.label("2. Memoriza cada figura y su color");
                    ui.label("3. Despues seleccionaras la forma y color de cada figura");
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Formas disponibles:");
                    ui.horizontal_wrapped(|ui| {
                        for shape in Shape::all() {
                            let fig = Figure { shape, color: ShapeColor::Blue };
                            ui.vertical(|ui| {
                                fig.render(ui, 40.0);
                                ui.label(shape.name());
                            });
                        }
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Colores disponibles:");
                    ui.horizontal_wrapped(|ui| {
                        for color in ShapeColor::all() {
                            let fig = Figure { shape: Shape::Circle, color };
                            ui.vertical(|ui| {
                                fig.render(ui, 30.0);
                                ui.label(color.name());
                            });
                        }
                    });
                });

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Configuracion:");
                    ui.label(format!("Modo: {}", self.config.mode.name()));
                    ui.label(format!("Figuras: {}", self.config.element_count));
                    ui.label(format!("Velocidad inicial: {}ms", self.config.initial_speed_ms));
                    if self.config.constant_time {
                        ui.label("Velocidad: Constante");
                    } else {
                        ui.label("Velocidad: Progresiva (aumenta con cada figura)");
                    }
                    if self.config.figure_blank_time_ms > 0 {
                        ui.label(format!("Tiempo en blanco entre figuras: {}ms", self.config.figure_blank_time_ms));
                    }
                    ui.label(format!("Intentos: {}", self.config.max_attempts));
                });

                ui.add_space(20.0);

                if ui.button("Comenzar").clicked() {
                    self.state = FigurasState::Ready;
                }
            }

            FigurasState::Ready => {
                self.draw_menu_button(ui);

                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.heading(format!("Intento {} de {}", self.current_attempt, self.config.max_attempts));
                    ui.add_space(20.0);
                    ui.label(format!("{} figuras", self.figures.len()));
                    ui.label(format!("Velocidad inicial: {}ms", self.config.initial_speed_ms));
                    if self.config.constant_time {
                        ui.label("Velocidad constante");
                    } else {
                        ui.label("La velocidad aumentara progresivamente");
                    }
                    if self.config.figure_blank_time_ms > 0 {
                        ui.label(format!("Tiempo en blanco: {}ms", self.config.figure_blank_time_ms));
                    }
                    ui.add_space(30.0);

                    if ui.button("Iniciar").clicked() {
                        self.start_showing();
                    }
                });
            }

            FigurasState::ShowingFigures => {
                if let Some(figure_start) = self.figure_start_time {
                    let elapsed = figure_start.elapsed();

                    if self.is_blank_phase {
                        // Fase de tiempo en blanco
                        let blank_time = self.get_blank_time();

                        if elapsed >= blank_time {
                            // Terminar fase de blank, mostrar siguiente figura
                            self.is_blank_phase = false;
                            self.figure_start_time = Some(Instant::now());
                            return;
                        }

                        // Mostrar pantalla en blanco
                        ui.vertical_centered(|ui| {
                            ui.add_space(80.0);
                            ui.label(format!(
                                "Figura {} de {}",
                                self.current_figure_index + 1,
                                self.figures.len()
                            ));
                            ui.add_space(50.0);
                            ui.label("...");
                        });
                    } else {
                        // Fase de mostrar figura
                        let display_time = self.get_display_time(self.current_figure_index);

                        if elapsed >= display_time {
                            // Avanzar a siguiente figura o fase blank
                            self.current_figure_index += 1;
                            if self.current_figure_index >= self.figures.len() {
                                self.state = FigurasState::InputFigures;
                                self.current_figure_index = 0;
                                self.current_answer_shape = None;
                                self.current_answer_color = None;
                                return;
                            }

                            // Si hay blank time, entrar en fase blank
                            if self.config.figure_blank_time_ms > 0 {
                                self.is_blank_phase = true;
                                self.figure_start_time = Some(Instant::now());
                            } else {
                                self.figure_start_time = Some(Instant::now());
                            }
                            return;
                        }

                        let remaining = display_time - elapsed;

                        ui.vertical_centered(|ui| {
                            ui.add_space(20.0);
                            ui.label(format!(
                                "Figura {} de {}",
                                self.current_figure_index + 1,
                                self.figures.len()
                            ));
                            ui.label(format!("Tiempo: {:.2}s", remaining.as_secs_f32()));
                            ui.add_space(30.0);

                            if let Some(figure) = self.figures.get(self.current_figure_index) {
                                figure.render(ui, 120.0);
                            }

                            ui.add_space(20.0);
                        });
                    }
                }
            }

            FigurasState::InputFigures => {
                self.draw_menu_button(ui);

                ui.heading(format!(
                    "Figura {} de {}",
                    self.current_figure_index + 1,
                    self.figures.len()
                ));
                ui.separator();
                ui.add_space(10.0);

                // Selección de forma
                ui.label("Selecciona la forma:");
                ui.horizontal_wrapped(|ui| {
                    for shape in Shape::all() {
                        let selected = self.current_answer_shape == Some(shape);
                        let fig = Figure { shape, color: ShapeColor::Gray };

                        ui.vertical(|ui| {
                            if ui.selectable_label(selected, "").clicked() {
                                self.current_answer_shape = Some(shape);
                            }
                            fig.render(ui, 50.0);
                            ui.label(shape.name());
                        });
                    }
                });

                ui.add_space(20.0);

                // Selección de color
                ui.label("Selecciona el color:");
                ui.horizontal_wrapped(|ui| {
                    for color in ShapeColor::all() {
                        let selected = self.current_answer_color == Some(color);
                        let fig = Figure { shape: Shape::Circle, color };

                        ui.vertical(|ui| {
                            if ui.selectable_label(selected, "").clicked() {
                                self.current_answer_color = Some(color);
                            }
                            fig.render(ui, 40.0);
                            ui.label(color.name());
                        });
                    }
                });

                ui.add_space(20.0);

                // Preview de la selección
                if let (Some(shape), Some(color)) = (self.current_answer_shape, self.current_answer_color) {
                    ui.group(|ui| {
                        ui.label("Tu seleccion:");
                        let preview = Figure { shape, color };
                        preview.render(ui, 60.0);
                        ui.label(format!("{} {}", color.name(), shape.name()));
                    });
                }

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if self.current_figure_index > 0 && ui.button("< Anterior").clicked() {
                        // Guardar respuesta actual
                        if let (Some(shape), Some(color)) = (self.current_answer_shape, self.current_answer_color) {
                            self.user_answers[self.current_figure_index] = Some(Figure { shape, color });
                        }
                        self.current_figure_index -= 1;
                        // Cargar respuesta anterior
                        if let Some(prev) = &self.user_answers[self.current_figure_index] {
                            self.current_answer_shape = Some(prev.shape);
                            self.current_answer_color = Some(prev.color);
                        } else {
                            self.current_answer_shape = None;
                            self.current_answer_color = None;
                        }
                    }

                    let can_continue = self.current_answer_shape.is_some() && self.current_answer_color.is_some();

                    if self.current_figure_index < self.figures.len() - 1 {
                        if ui.add_enabled(can_continue, egui::Button::new("Siguiente >")).clicked() {
                            // Guardar respuesta
                            if let (Some(shape), Some(color)) = (self.current_answer_shape, self.current_answer_color) {
                                self.user_answers[self.current_figure_index] = Some(Figure { shape, color });
                            }
                            self.current_figure_index += 1;
                            // Cargar siguiente respuesta si existe
                            if let Some(next) = &self.user_answers[self.current_figure_index] {
                                self.current_answer_shape = Some(next.shape);
                                self.current_answer_color = Some(next.color);
                            } else {
                                self.current_answer_shape = None;
                                self.current_answer_color = None;
                            }
                        }
                    } else if ui.add_enabled(can_continue, egui::Button::new("Finalizar")).clicked() {
                        // Guardar última respuesta
                        if let (Some(shape), Some(color)) = (self.current_answer_shape, self.current_answer_color) {
                            self.user_answers[self.current_figure_index] = Some(Figure { shape, color });
                        }
                        // Calcular puntuación
                        let score = self.calculate_score();
                        if score > self.best_score {
                            self.best_score = score;
                        }
                        self.state = FigurasState::FinalResults;
                    }
                });

                // Navegación rápida
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.label("Ir a figura:");
                    for i in 0..self.figures.len() {
                        let answered = self.user_answers[i].is_some();
                        let label = if answered {
                            format!("[{}]", i + 1)
                        } else {
                            format!("{}", i + 1)
                        };
                        if ui.selectable_label(i == self.current_figure_index, label).clicked() {
                            // Guardar respuesta actual antes de cambiar
                            if let (Some(shape), Some(color)) = (self.current_answer_shape, self.current_answer_color) {
                                self.user_answers[self.current_figure_index] = Some(Figure { shape, color });
                            }
                            self.current_figure_index = i;
                            // Cargar respuesta si existe
                            if let Some(fig) = &self.user_answers[i] {
                                self.current_answer_shape = Some(fig.shape);
                                self.current_answer_color = Some(fig.color);
                            } else {
                                self.current_answer_shape = None;
                                self.current_answer_color = None;
                            }
                        }
                    }
                });
            }

            FigurasState::FinalResults => {
                self.draw_menu_button(ui);

                ui.heading("Resultados de Figuras de Colores");
                ui.separator();
                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label(RichText::new(format!("Figuras correctas: {}/{}", self.best_score, self.figures.len())).size(28.0).strong());
                    ui.add_space(10.0);
                    ui.label(format!(
                        "Precision: {:.1}%",
                        self.best_score as f32 / self.figures.len() as f32 * 100.0
                    ));
                    ui.label(format!("Velocidad inicial: {}ms", self.config.initial_speed_ms));
                });

                ui.add_space(10.0);

                // Mostrar comparación
                ui.collapsing("Ver comparacion", |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for (i, (original, answer)) in self.figures.iter().zip(self.user_answers.iter()).enumerate() {
                            let correct = answer.map(|a| a == *original).unwrap_or(false);
                            let status = if correct { "OK" } else { "X" };

                            ui.horizontal(|ui| {
                                ui.label(format!("#{} [{}]", i + 1, status));
                                ui.group(|ui| {
                                    ui.label("Original:");
                                    original.render(ui, 30.0);
                                });
                                if let Some(ans) = answer {
                                    ui.group(|ui| {
                                        ui.label("Tu respuesta:");
                                        ans.render(ui, 30.0);
                                    });
                                } else {
                                    ui.label("Sin respuesta");
                                }
                            });
                            ui.separator();
                        }
                    });
                });

                ui.add_space(20.0);

                if self.current_attempt < self.config.max_attempts {
                    if ui.button("Siguiente intento").clicked() {
                        self.current_attempt += 1;
                        // Regenerar figuras
                        self.figures = (0..self.config.element_count)
                            .map(|_| Figure::random())
                            .collect();
                        self.user_answers = vec![None; self.config.element_count];
                        self.current_figure_index = 0;
                        self.current_answer_shape = None;
                        self.current_answer_color = None;
                        self.state = FigurasState::Ready;
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

        Some(GameResult {
            game_type: GameType::FigurasColores,
            game_mode: self.config.mode,
            score: self.best_score as f32,
            details: GameDetails::FigurasColores {
                figures_correct: self.best_score,
                total_figures: self.figures.len(),
                initial_speed_ms: self.config.initial_speed_ms,
                points: self.best_score,
                attempt_number: self.current_attempt,
            },
            timestamp: std::time::SystemTime::now(),
        })
    }

    fn needs_repaint(&self) -> bool {
        matches!(self.state, FigurasState::ShowingFigures)
    }
}
