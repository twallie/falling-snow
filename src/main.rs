use std::{thread, time::{self, SystemTime}};

use rand::Rng;
use termgrid::{grid::Grid, print::{self, Printer}};

#[derive(Clone, Copy, Eq, PartialEq)]
enum CellState {
    Snow,
    Empty
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CellState::Empty => " ",
            CellState::Snow => "*"
        };
        write!(f, "{}", str)
    }
}

fn main() {
    let grid = Grid::new(CellState::Empty);
    let mut printer = Printer::new(grid.clone());
    printer.start();

    // lets just start by creating a random grid
    loop {
        let new_grid = random_snow_grid();
        printer = match printer.update(new_grid) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let time = time::Duration::from_millis(1);
        thread::sleep(time);
    }
}

fn random_snow_grid() -> Grid<CellState> {
    let mut grid = Grid::new(CellState::Empty);
    
    let row_count = grid.grid.len();
    let column_count = grid.grid[0].len();

    for row_index in 0..row_count {
        for column_index in 0..column_count {
            if fifty_fifty_chance() {
                grid.grid[row_index][column_index] = CellState::Snow;
            }
        }
    }

    return grid;
}

fn fifty_fifty_chance() -> bool {
    let num = rand::thread_rng().gen_range(0..3);
    return (num % 3) == 0;
}
