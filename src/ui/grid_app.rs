use crate::ui::main_grid::MainGrid;
use crate::ui::sub_grid::SubGrid;
use eframe::egui;
use egui::{Key, Pos2, Vec2};
use enigo::Coordinate::Abs;
use enigo::{Button, Direction, Mouse, Settings};
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

pub struct GridApp {
    // TODO: Split state out into MainGrid and SubGrid
    // TODO: Implement a trait like "Renderer" which they both implement
    // TODO: Split key handling into two Structs and create a trait like "KeyHandler" they both implement
    // TODO: Consider using a lot more Result<()> return values. This way we can match on them
    label_positions: HashMap<String, Pos2>,
    first_key: Option<Key>,
    should_render_sub_grid: bool,
    should_click: bool,
    sub_grid_position: Option<Pos2>,
    cell_size: Option<Vec2>,
    enigo: enigo::Enigo,
}

impl GridApp {
    pub fn new() -> GridApp {
        Self {
            label_positions: HashMap::new(),
            first_key: None,
            should_render_sub_grid: false,
            should_click: false,
            sub_grid_position: None,
            cell_size: None,
            enigo: enigo::Enigo::new(&Settings::default())
                .expect("Failed to create Enigo instance"),
        }
    }

    pub fn add_position(&mut self, label: String, position: Pos2) {
        self.label_positions.insert(label, position);
    }

    pub fn set_cell_size(&mut self, size: Vec2) {
        self.cell_size = Some(size);
    }

    fn move_mouse_to(&mut self, pos: Pos2) {
        self.enigo
            .move_mouse(pos.x.round() as i32, pos.y.round() as i32, Abs)
            .expect("Failed to move mouse");
    }

    fn click_mouse(&mut self) {
        println!("Clicking..");
        self.enigo
            .button(Button::Left, Direction::Click)
            .expect("Failed to click");
    }

    fn render_main_grid(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for key in HOTKEYS {
            let pressed_key = key.name();

            if ctx.input(|i| i.key_pressed(*key)) {
                let Some(first_key) = self.first_key else {
                    println!("First key pressed: {pressed_key}");
                    self.first_key = Some(*key);
                    break;
                };

                let key_combo = format!("{}{}", first_key.name(), pressed_key);
                println!("Key combo: {key_combo}");

                if let Some(&position) = self.label_positions.get(&key_combo) {
                    println!("Moving to {position}");
                    self.move_mouse_to(position);
                    self.sub_grid_position = Some(position);
                    self.should_render_sub_grid = true;
                } else {
                    println!("Invalid key combo");
                }

                break;
            }
        }

        self.label_positions.clear();

        MainGrid::render(self, ctx);
    }

    fn render_sub_grid(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for key in NUMBER_KEYS {
            if ctx.input(|i| i.key_pressed(*key)) {
                println!("Key pressed: {}", key.name());

                if let Some(&position) = self.label_positions.get(key.name()) {
                    println!("Moving to {position}");
                    self.move_mouse_to(position);
                    ctx.send_viewport_cmd(egui::ViewportCommand::MousePassthrough(true));
                    self.should_click = true;
                } else {
                    println!("Invalid key");
                }
                break;
            }
        }

        self.label_positions.clear();

        let outer_cell_size = self.cell_size.expect("Should have a cell size");
        let center_position = self
            .sub_grid_position
            .expect("Should have a sub grid position");
        let sub_grid = SubGrid::new(center_position, outer_cell_size);

        sub_grid.render(self, ctx);
    }
}

impl eframe::App for GridApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.should_click {
            self.should_click = false;
            self.click_mouse();
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        if self.should_render_sub_grid {
            self.render_sub_grid(ctx, frame);
        } else {
            self.render_main_grid(ctx, frame);
        }
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // fully transparent background color
    }
}
