use crate::ui::main_grid::MainGrid;
use crate::ui::renderer::{KeyResult, Renderer};
use eframe::egui;
use egui::{Key, Pos2};
use enigo::Coordinate::Abs;
use enigo::{Button, Direction, Mouse, Settings};

pub struct App {
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
            .button(Button::Left, Direction::Click)
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
                self.move_mouse_to(mouse_position)
            }
            Ok(KeyResult::Click { position }) => {
                self.move_mouse_to(position);
                ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
                self.should_click = true;
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
