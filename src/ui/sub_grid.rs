use crate::ui::renderer::KeyResult::Await;
use crate::ui::renderer::{KeyResult, Renderer};
use eframe::emath::{Align2, Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Stroke};
use egui::{Context, Frame, Key};
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

                for column in 0..3 {
                    for row in 0..3 {
                        let x = outer_x + (self.cell_size.x * row as f32);
                        let y = outer_y + (self.cell_size.y * column as f32);

                        let rect = Rect::from_min_size(Pos2::new(x, y), self.cell_size);

                        painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);

                        painter.text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            i,
                            egui::FontId::proportional((self.cell_size.y * 0.35).clamp(10.0, 48.0)),
                            Color32::WHITE,
                        );

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
        for key in NUMBER_KEYS {
            if ctx.input(|i| i.key_pressed(*key)) {
                println!("Key pressed: {}", key.name());

                if let Some(&position) = self.label_positions.get(key.name()) {
                    return Ok(KeyResult::Click { position });
                }

                println!("Invalid key");
                break;
            }
        }

        Ok(Await)
    }
}
