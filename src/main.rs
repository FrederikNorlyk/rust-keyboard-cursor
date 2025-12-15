use eframe::egui;
use egui::{Align2, Color32, Pos2, Rect, Stroke, Vec2};

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

struct GridApp;

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
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
                    painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Outside);

                    // Cell label
                    let label = index_to_label(id);
                    id = id.saturating_add(1);

                    painter.text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        label,
                        egui::FontId::proportional((cell_h * 0.35).clamp(10.0, 48.0)),
                        Color32::from_gray(230),
                    );
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Grid AA..ZZ Overlay"),
        ..Default::default()
    };

    eframe::run_native(
        "Grid AA..ZZ Overlay",
        native_options,
        Box::new(|_cc| Ok(Box::new(GridApp))),
    )
}
