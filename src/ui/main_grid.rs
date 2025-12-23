use crate::ui::grid_app::GridApp;
use eframe::emath::{Align2, Pos2, Rect, Vec2};
use eframe::epaint::{Color32, Stroke};
use egui::Frame;

pub struct MainGrid;

impl MainGrid {
    pub fn render(app: &mut GridApp, ctx: &egui::Context) {
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

                app.set_cell_size(cell_size);

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

                        app.add_position(label.clone(), rect.center());

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
