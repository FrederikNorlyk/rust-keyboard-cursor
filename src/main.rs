#![windows_subsystem = "windows"]

use crate::ui::app::App;

mod ui;

fn is_hyprland() -> bool {
    std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
        || std::env::var("XDG_CURRENT_DESKTOP")
            .map(|v| v.to_lowercase() == "hyprland")
            .unwrap_or(false)
}

fn main() -> eframe::Result {
    let app_id = "keyboard-cursor";

    let mut viewport_builder = egui::ViewportBuilder::default()
        .with_title("Keyboard cursor")
        .with_app_id(app_id)
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top();

    if is_hyprland() {
        // Avoid using .with_fullscreen as Hyprland will hide the other windows.
        viewport_builder = viewport_builder.with_inner_size([10000.0, 10000.0]);
    } else {
        viewport_builder = viewport_builder.with_fullscreen(true);
    }

    let native_options = eframe::NativeOptions {
        viewport: viewport_builder,
        ..Default::default()
    };

    eframe::run_native(
        app_id,
        native_options,
        Box::new(|_| Ok(Box::new(App::new()))),
    )
}
