#![windows_subsystem = "windows"]

use crate::ui::main_grid::MainGrid;
use crate::ui::renderer::{Direction, KeyResult, Renderer};
use eframe::egui;
use egui::{Key, Pos2};
use enigo::Coordinate::{Abs, Rel};
use enigo::{Button, Mouse, Settings};

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

struct App {
    renderer: Box<dyn Renderer>,
    should_click: bool,
    is_closing: bool,
    enigo: enigo::Enigo,
}

impl App {
    pub fn new() -> App {
        Self {
            renderer: Box::new(MainGrid::new()),
            should_click: false,
            is_closing: false,
            enigo: enigo::Enigo::new(&Settings::default())
                .expect("Failed to create Enigo instance"),
        }
    }

    fn move_mouse_to(&mut self, position: Pos2) {
        println!("Moving mouse to {position}");

        self.enigo
            .move_mouse(position.x.round() as i32, position.y.round() as i32, Abs)
            .expect("Failed to move mouse");
    }

    fn click_mouse(&mut self) {
        println!("Clicking..");
        self.enigo
            .button(Button::Left, enigo::Direction::Click)
            .expect("Failed to click");
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.is_closing {
            println!("Is closing");
            return;
        }

        if self.should_click {
            println!("Should click");
            self.click_mouse();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            self.is_closing = true;
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            println!("Escape clicked");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            self.is_closing = true;
            return;
        }

        match self.renderer.await_key(ctx) {
            Ok(KeyResult::Await) => {}
            Ok(KeyResult::SetRenderer {
                renderer,
                mouse_position,
            }) => {
                self.renderer = renderer;
                self.move_mouse_to(mouse_position);
            }
            Ok(KeyResult::MoveAndClick { position }) => {
                self.move_mouse_to(position);
                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
                self.should_click = true;
            }
            Ok(KeyResult::Click) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
                self.should_click = true;
            }
            Ok(KeyResult::Move { direction, speed }) => {
                let (dx, dy) = match direction {
                    Direction::Up => (0, -speed),
                    Direction::Down => (0, speed),
                    Direction::Left => (-speed, 0),
                    Direction::Right => (speed, 0),
                };

                self.enigo
                    .move_mouse(dx, dy, Rel)
                    .expect("Failed to move mouse relatively");
            }
            Err(e) => {
                eprintln!("Key handler error: {e}");
            }
        }

        self.renderer.render(ctx);
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // fully transparent background color
    }
}
