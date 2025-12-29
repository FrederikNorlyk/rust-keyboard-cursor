use egui::Pos2;

pub enum KeyResult {
    Await,
    SetRenderer {
        renderer: Box<dyn Renderer>,
        mouse_position: Pos2,
    },
    MoveAndClick {
        position: Pos2,
    },
    Move {
        direction: Direction,
        speed: i32,
    },
    Click,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub trait Renderer {
    fn render(&mut self, ctx: &egui::Context);

    fn get_label_position(&self, label: String) -> Option<&Pos2>;

    fn await_key(&mut self, ctx: &egui::Context) -> Result<KeyResult, String>;
}
