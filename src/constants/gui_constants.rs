use macroquad::{color::{Color, GRAY}, window::Conf};

pub const WINDOW_SIZE_X: usize = 600;
pub const WINDOW_SIZE_Y: usize = 600;

pub const CELL_COLOR: Color = GRAY;

pub fn get_window_config() -> Conf {
    Conf {
        window_title: "Life".to_owned(),
        window_width: WINDOW_SIZE_X as i32,
        window_height: WINDOW_SIZE_Y as i32,
        fullscreen: false,
        ..Default::default()
    }
}
