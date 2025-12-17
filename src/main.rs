use eframe::egui;
use egui::{Align2, Color32, Frame, Key, Pos2, Rect, Stroke, Vec2};
use enigo::Coordinate::Abs;
use enigo::{Mouse, Settings};
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


struct GridApp {
    label_positions: HashMap<String, Pos2>,
    first_key: Option<Key>,
    enigo: enigo::Enigo,
}

impl GridApp {
    fn move_mouse_to(&mut self, pos: Pos2) {
        self.enigo
            .move_mouse(pos.x.round() as i32, pos.y.round() as i32, Abs)
            .expect("Failed to move mouse");
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        for key in HOTKEYS {
            let character = key.name().chars().next().unwrap();

            if ctx.input(|i| i.key_pressed(*key)) {
                let Some(first_key) = self.first_key else {
                    println!("First key pressed: {}", character);
                    self.first_key = Some(*key);
                    break;
                };

                let key_combo = format!("{}{}", first_key.name(), character);
                println!("Key combo: {}", key_combo);

                if let Some(&position) = self.label_positions.get(&key_combo) {
                    println!("Moving to {}:{}", position.x, position.y);
                    self.move_mouse_to(position);
                } else {
                    println!("Invalid key combo");
                }

                self.first_key = None;
                break;
            }
        }

        self.label_positions.clear();

        egui::CentralPanel::default()
            .frame(Frame::NONE)
            .show(ctx, |ui| {
                // Use all available space
                let available = ui.available_size();
                let top_left = ui.min_rect().min;

                // Decide grid size: target cell size ~80x80, but fit evenly
                let target = 80.0_f32;
                let cols = (available.x / target).floor().max(1.0) as usize;
                let rows = (available.y / target).floor().max(1.0) as usize;

                let cell_w = if cols > 0 {
                    available.x / cols as f32
                } else {
                    available.x
                };

                let cell_h = if rows > 0 {
                    available.y / rows as f32
                } else {
                    available.y
                };

                let painter = ui.painter_at(Rect::from_min_size(top_left, available));

                let stroke = Stroke {
                    width: 1.0,
                    color: Color32::from_gray(100),
                };

                let mut id = 0_usize;

                for row in 0..rows {
                    for column in 0..cols {
                        let x = top_left.x + column as f32 * cell_w;
                        let y = top_left.y + row as f32 * cell_h;

                        let rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(cell_w, cell_h));

                        // Cell border
                        let transparent = Color32::from_rgba_unmultiplied(0, 0, 0, 5);
                        painter.rect_filled(rect, 0, transparent);
                        painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Outside);

                        // Cell label
                        let label = index_to_label(id);
                        let center = rect.center();

                        self.label_positions.insert(label.clone(), center);

                        id = id.saturating_add(1);

                        painter.text(
                            center,
                            Align2::CENTER_CENTER,
                            label,
                            egui::FontId::proportional((cell_h * 0.35).clamp(10.0, 48.0)),
                            Color32::WHITE,
                        );
                    }
                }
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // fully transparent background color
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

fn main() -> eframe::Result {
    let app_id = "keyboard-cursor";

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Keyboard cursor")
            .with_app_id(app_id)
            .with_decorations(false)
            .with_transparent(true)
            // Avoid using .with_fullscreen as Hyprland will hide the other windows.
            .with_inner_size([10000.0, 10000.0]),

        ..Default::default()
    };

    let app = GridApp {
        label_positions: HashMap::new(),
        first_key: None,
        enigo: enigo::Enigo::new(&Settings::default()).expect("Failed to create Enigo instance"),
    };
    eframe::run_native(app_id, native_options, Box::new(|_| Ok(Box::new(app))))
}
