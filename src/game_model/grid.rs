type GridDimensionsIntegerType = u16;

use crate::game_model::cell;

pub struct Grid {
    number_of_cells_x: GridDimensionsIntegerType,
    number_of_cells_y: GridDimensionsIntegerType,

    cells: Vec<Vec<cell::Cell>>,

    t: usize,
}

impl Grid {
    pub fn new(
        number_of_cells_x: GridDimensionsIntegerType,
        number_of_cells_y: GridDimensionsIntegerType,
    ) -> Grid {
        let mut v: Vec<Vec<cell::Cell>> = Vec::new();
        for y in 0..number_of_cells_y {
            v.push(Vec::new());
            for x in 0..number_of_cells_x {
                v[y as usize].push(cell::Cell::new(x as usize, y as usize));
            }
        }

        Self {
            number_of_cells_x: number_of_cells_x,
            number_of_cells_y: number_of_cells_y,
            cells: v,
            t: 0,
        }
    }

    pub fn init(&mut self) {
        for y in 0..self.number_of_cells_y {
            for x in 0..self.number_of_cells_x {
                if y == x || y == x + 1 || (x >= 2 && y == x - 2) {
                    self.cells[y as usize][x as usize].set_alive(true);
                }
            }
        }
    }

    pub fn update_cells(&mut self) {
        fn count_neighbors(cell: &cell::Cell, cells: &Vec<Vec<cell::Cell>>) -> usize {
            let mut alive_neighbors: usize = 0;
            let x = cell.get_x();
            let y = cell.get_y();
            if x < cells[0].len() - 1 {
                if cells[y][x + 1].is_alive() {
                    alive_neighbors += 1;
                }
                if y < cells.len() - 1 {
                    if cells[y + 1][x + 1].is_alive() {
                        alive_neighbors += 1;
                    }
                }
                if y > 0 {
                    if cells[y - 1][x + 1].is_alive() {
                        alive_neighbors += 1;
                    }
                }
            }
            if x > 0 {
                if cells[y][x - 1].is_alive() {
                    alive_neighbors += 1;
                }
                if y < cells.len() - 1 {
                    if cells[y + 1][x - 1].is_alive() {
                        alive_neighbors += 1;
                    }
                }
                if y > 0 {
                    if cells[y - 1][x - 1].is_alive() {
                        alive_neighbors += 1;
                    }
                }
            }
            if y < cells.len() - 1 {
                if cells[y + 1][x].is_alive() {
                    alive_neighbors += 1;
                }
            }
            if y > 0 {
                if cells[y - 1][x].is_alive() {
                    alive_neighbors += 1;
                }
            }
            alive_neighbors
        }

        let mut mask: Vec<Vec<bool>> = Vec::new();

        for y in 0..self.number_of_cells_y {
            mask.push(Vec::new());
            for x in 0..self.number_of_cells_x {
                let number_of_neighbors =
                    count_neighbors(&self.cells[y as usize][x as usize], &self.cells);

                mask[y as usize].push(self.cells[y as usize][x as usize].is_alive());
                // if not enough cells, die
                if number_of_neighbors < 2 {
                    mask[y as usize][x as usize] = false;
                }

                // if too many, die
                if number_of_neighbors > 3 {
                    mask[y as usize][x as usize] = false;
                }

                // if precisely 3 cell around, live
                if number_of_neighbors == 3 {
                    mask[y as usize][x as usize] = true;
                }
            }
        }

        self.cells.iter_mut().for_each(|cell_row| {
            cell_row
                .iter_mut()
                .for_each(|cell| cell.set_alive(mask[cell.get_y()][cell.get_x()]))
        });
        self.t += 1;
    }

    pub fn get_cells(&self) -> &Vec<Vec<cell::Cell>> {
        &self.cells
    }

    pub fn get_number_of_cells_x_y(
        &self,
    ) -> (GridDimensionsIntegerType, GridDimensionsIntegerType) {
        (self.number_of_cells_x, self.number_of_cells_y)
    }
}
