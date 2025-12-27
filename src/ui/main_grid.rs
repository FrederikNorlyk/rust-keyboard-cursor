use crate::ui::renderer::{KeyResult, Renderer};
use crate::ui::sub_grid::SubGrid;
use eframe::emath::{Align2, Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Stroke};
use egui::{Context, Frame, Key};
use std::collections::HashMap;

const HOTKEYS: &[Key] = &[
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
];

pub struct MainGrid {
    label_positions: HashMap<String, Pos2>,
    cell_size: Option<Vec2>,
    first_key: Option<Key>,
}

impl MainGrid {
    pub fn new() -> MainGrid {
        MainGrid {
            label_positions: HashMap::new(),
            cell_size: None,
            first_key: None,
        }
    }
}

impl Renderer for MainGrid {
    fn render(&mut self, ctx: &Context) {
        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                let screen_size = ui.available_size();
                let top_left = ui.min_rect().min;

                // Decide grid size: target cell size ~80x80, but fit evenly
                let target_cell_size = 80.0_f32;
                let cols = (screen_size.x / target_cell_size).floor().max(1.0) as usize;
                let rows = (screen_size.y / target_cell_size).floor().max(1.0) as usize;
                let cell_width = screen_size.x / cols as f32;
                let cell_height = screen_size.y / rows as f32;
                let cell_size = Vec2::new(cell_width, cell_height);

                self.cell_size = Some(cell_size);

                let painter = ui.painter_at(Rect::from_min_size(top_left, screen_size));

                let stroke = Stroke {
                    width: 0.3,
                    color: Color32::from_gray(100),
                };

                let mut id = 0_usize;

                for row in 0..rows {
                    for column in 0..cols {
                        let x = top_left.x + column as f32 * cell_width;
                        let y = top_left.y + row as f32 * cell_height;

                        let rect = Rect::from_min_size(Pos2::new(x, y), cell_size);

                        painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Outside);

                        let label = index_to_label(id);

                        self.label_positions.insert(label.clone(), rect.center());

                        painter.text(
                            Pos2::new(rect.center().x - 2_f32, rect.center().y - 2_f32),
                            Align2::CENTER_CENTER,
                            &label,
                            egui::FontId::proportional((cell_height * 0.35).clamp(10.0, 48.0)),
                            Color32::BLACK,
                        );
                        painter.text(
                            rect.center(),
                            Align2::CENTER_CENTER,
                            label,
                            egui::FontId::proportional((cell_height * 0.35).clamp(10.0, 48.0)),
                            Color32::WHITE,
                        );

                        id = id.saturating_add(1);
                    }
                }
            });
    }

    fn get_label_position(&self, label: String) -> Option<&Pos2> {
        self.label_positions.get(&label)
    }

    fn await_key(&mut self, ctx: &Context) -> Result<KeyResult, String> {
        for key in HOTKEYS {
            if !ctx.input(|i| i.key_pressed(*key)) {
                continue;
            }

            let pressed_key = key.name();

            let Some(first_key) = self.first_key else {
                println!("First key pressed: {pressed_key}");
                self.first_key = Some(*key);
                break;
            };

            let key_combo = format!("{}{}", first_key.name(), pressed_key);
            println!("Key combo: {key_combo}");

            if let Some(&position) = self.get_label_position(key_combo)
                && let Some(cell_size) = self.cell_size
            {
                return Ok(KeyResult::SetRenderer {
                    renderer: Box::new(SubGrid::new(position, cell_size)),
                    mouse_position: position,
                });
            } else {
                // TODO: Reset the first_key?
                println!("Invalid key combo");
            }

            break;
        }

        Ok(KeyResult::Await)
    }
}

/// Convert a zero-based index to a two-letter code from `AA` to `ZZ`.
/// 0 -> AA, 1 -> AB, ..., 25 -> AZ, 26 -> BA, ..., 675 -> ZZ.
/// Values beyond 675 wrap around.
fn index_to_label(mut idx: usize) -> String {
    // 26 * 26 = 676 combinations
    idx %= 26 * 26;
    let first = (idx / 26) as u8;
    let second = (idx % 26) as u8;
    let a = b'A';
    let s = [a + first, a + second];
    // SAFETY: ASCII uppercase letters
    String::from_utf8(s.to_vec()).expect("valid ASCII")
}
