use eframe::emath::{Align2, Pos2};
use eframe::epaint::Color32;
use egui::Painter;

pub struct Text;

impl Text {
    pub fn draw(painter: &Painter, position: Pos2, text: &impl ToString, size: f32) {
        let text = text.to_string();

        // Text shadow
        painter.text(
            Pos2::new(position.x + 1_f32, position.y + 1_f32),
            Align2::CENTER_CENTER,
            &text,
            egui::FontId::proportional(size),
            Color32::BLACK,
        );

        painter.text(
            position,
            Align2::CENTER_CENTER,
            text,
            egui::FontId::proportional(size),
            Color32::WHITE,
        );
    }
}
