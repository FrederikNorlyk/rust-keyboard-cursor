use crate::ui::renderer::KeyResult::Await;
use crate::ui::renderer::{Direction, KeyResult, Renderer};
use crate::ui::text::Text;
use eframe::emath::{Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Stroke};
use egui::{Context, Frame, InputState, Key};
use std::collections::HashMap;

const NUMBER_KEYS: &[Key] = &[
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
];

const MOVE_LEFT_KEYS: &[Key] = &[Key::ArrowLeft, Key::H];
const MOVE_UP_KEYS: &[Key] = &[Key::ArrowUp, Key::K];
const MOVE_DOWN_KEYS: &[Key] = &[Key::ArrowDown, Key::J];
const MOVE_RIGHT_KEYS: &[Key] = &[Key::ArrowRight, Key::L];

pub struct SubGrid {
    outer_rect: Rect,
    cell_size: Vec2,
    label_positions: HashMap<String, Pos2>,
}

impl SubGrid {
    pub fn new(center_position: Pos2, outer_cell_size: Vec2) -> SubGrid {
        let inner_width = outer_cell_size.x / 3.0;
        let inner_height = outer_cell_size.y / 3.0;

        Self {
            outer_rect: Rect::from_center_size(center_position, outer_cell_size),
            cell_size: Vec2::new(inner_width, inner_height),
            label_positions: HashMap::new(),
        }
    }
}

impl Renderer for SubGrid {
    fn render(&mut self, ctx: &Context) {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                let painter = ui.painter_at(self.outer_rect);

                let stroke = Stroke {
                    width: 1.0,
                    color: Color32::from_gray(100),
                };

                let outer_x = self.outer_rect.min.x;
                let outer_y = self.outer_rect.min.y;

                let mut i = 1;

                let text_size = (self.cell_size.y * 0.6).clamp(10.0, 48.0);

                for column in 0..3 {
                    for row in 0..3 {
                        let x = outer_x + (self.cell_size.x * row as f32);
                        let y = outer_y + (self.cell_size.y * column as f32);

                        let rect = Rect::from_min_size(Pos2::new(x, y), self.cell_size);

                        if i % 2 == 0 {
                            painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);
                        }

                        Text::draw(&painter, rect.center(), &i, text_size);

                        self.label_positions.insert(format!("{i}"), rect.center());
                        i += 1;
                    }
                }
            });
    }

    fn get_label_position(&self, label: String) -> Option<&Pos2> {
        self.label_positions.get(&label)
    }

    fn await_key(&mut self, ctx: &Context) -> Result<KeyResult, String> {
        let input = ctx.input(Clone::clone);

        for key in NUMBER_KEYS {
            if input.key_pressed(*key) {
                println!("Key pressed: {}", key.name());

                if let Some(&position) = self.label_positions.get(key.name()) {
                    return Ok(KeyResult::MoveAndClick { position });
                }

                println!("Invalid key");
                break;
            }
        }

        // TODO: Change usage of key_pressed to key_released where you don't hold down the key
        if input.key_pressed(Key::Space) {
            return Ok(KeyResult::Click);
        }

        for key in MOVE_LEFT_KEYS {
            if input.key_pressed(*key) {
                return Ok(KeyResult::Move {
                    direction: Direction::Left,
                    speed: get_speed(&input),
                });
            }
        }

        for key in MOVE_UP_KEYS {
            if input.key_pressed(*key) {
                return Ok(KeyResult::Move {
                    direction: Direction::Up,
                    speed: get_speed(&input),
                });
            }
        }

        for key in MOVE_DOWN_KEYS {
            if input.key_pressed(*key) {
                return Ok(KeyResult::Move {
                    direction: Direction::Down,
                    speed: get_speed(&input),
                });
            }
        }

        for key in MOVE_RIGHT_KEYS {
            if input.key_pressed(*key) {
                return Ok(KeyResult::Move {
                    direction: Direction::Right,
                    speed: get_speed(&input),
                });
            }
        }

        Ok(Await)
    }
}

fn get_speed(input_state: &InputState) -> i32 {
    if input_state.modifiers.shift { 20 } else { 5 }
}
