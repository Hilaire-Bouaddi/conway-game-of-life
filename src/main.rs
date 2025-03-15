mod game_model;
mod constants {
    pub mod game_constants;
    pub mod gui_constants;
}
use core::time;
use macroquad::prelude::*;
use std::thread;

use constants::gui_constants::get_window_config;

fn game_coord_to_window_coord(x: usize, y: usize, grid: &game_model::grid::Grid) -> (usize, usize) {
    let (number_of_cells_x, number_of_cells_y) = grid.get_number_of_cells_x_y();
    let window_x: usize = x * constants::gui_constants::WINDOW_SIZE_X / number_of_cells_x as usize;
    let window_y: usize = y * constants::gui_constants::WINDOW_SIZE_Y / number_of_cells_y as usize;

    (window_x, window_y)
}

fn display_grid(grid: &game_model::grid::Grid) {
    for cell_row in grid.get_cells() {
        for cell in cell_row {
            if cell.is_alive() {
                let (x, y) = game_coord_to_window_coord(cell.get_x(), cell.get_y(), grid);
                let rect_width = constants::gui_constants::WINDOW_SIZE_X
                    / constants::game_constants::NUMBER_OF_CELLS_X as usize;
                let rect_height = constants::gui_constants::WINDOW_SIZE_Y
                    / constants::game_constants::NUMBER_OF_CELLS_Y as usize;

                draw_rectangle(
                    x as f32,
                    y as f32,
                    rect_width as f32,
                    rect_height as f32,
                    constants::gui_constants::CELL_COLOR,
                );
            }
        }
    }
}

#[macroquad::main(get_window_config)]
async fn main() {
    let mut grid = game_model::grid::Grid::new(
        constants::game_constants::NUMBER_OF_CELLS_X,
        constants::game_constants::NUMBER_OF_CELLS_Y,
    );
    grid.init();

    loop {
        clear_background(BLACK);

        display_grid(&grid);
        grid.update_cells();

        let one_sec =
            time::Duration::from_millis(constants::game_constants::REFRESH_RATE_MS as u64);
        thread::sleep(one_sec);

        next_frame().await
    }
}
